// TODO Reviewed ??-??-2026.

#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! A minimal Kubernetes operator with `kube-rs`: a custom resource (`Echo`),
//! a pure function that computes the desired `Deployment`, and a controller
//! loop that applies it and reports the status.

use std::sync::Arc;
use std::time::Duration;

use futures::StreamExt as _;
use k8s_openapi::api::apps::v1::Deployment;
use kube::Api;
use kube::Client;
use kube::CustomResource;
use kube::CustomResourceExt as _;
use kube::Resource as _;
use kube::ResourceExt as _;
use kube::api::Patch;
use kube::api::PatchParams;
use kube::runtime::controller::Action;
use kube::runtime::controller::Controller;
use kube::runtime::watcher::Config;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

/// The desired state of an `Echo`.
///
/// `#[derive(CustomResource)]` generates the `Echo` type (with `metadata`,
/// `spec` and `status`) and its CRD.
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "example.com",
    version = "v1",
    kind = "Echo",
    namespaced,
    status = "EchoStatus"
)]
pub struct EchoSpec {
    /// The message the pods echo.
    pub message: String,
    /// The number of pods.
    pub replicas: i32,
}

/// The observed state of an `Echo`, written to the status subresource.
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct EchoStatus {
    /// The number of ready pods.
    pub ready_replicas: i32,
}

/// Reconciliation errors.
#[derive(Debug, thiserror::Error)]
pub enum ReconcileError {
    /// The `Echo` has no name or UID, so it cannot own a `Deployment`.
    #[error("the Echo is missing metadata")]
    MissingMetadata,
    /// The desired `Deployment` could not be built.
    #[error("invalid Deployment: {0}")]
    Invalid(#[from] serde_json::Error),
    /// The API server rejected a request.
    #[error("Kubernetes API error: {0}")]
    Kube(#[from] kube::Error),
}

/// Shared state for the reconciler.
struct Context {
    client: Client,
}

/// A pure function from the spec to the desired `Deployment`: it is
/// idempotent and can be unit-tested without a cluster.
pub fn desired_deployment(echo: &Echo) -> Result<Deployment, ReconcileError> {
    let name = echo.name_any();
    // The owner reference makes Kubernetes garbage-collect the Deployment
    // when the `Echo` is deleted.
    let owner = echo
        .controller_owner_ref(&())
        .ok_or(ReconcileError::MissingMetadata)?;
    Ok(serde_json::from_value(json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": { "name": name, "ownerReferences": [owner] },
        "spec": {
            "replicas": echo.spec.replicas,
            "selector": { "matchLabels": { "app": name } },
            "template": {
                "metadata": { "labels": { "app": name } },
                "spec": { "containers": [{
                    "name": "echo",
                    "image": "hashicorp/http-echo",
                    "args": [format!("-text={}", echo.spec.message)]
                }] }
            }
        }
    }))?)
}

/// Brings the cluster closer to the desired state, then reports the status.
async fn reconcile(
    echo: Arc<Echo>,
    ctx: Arc<Context>,
) -> Result<Action, ReconcileError> {
    let namespace = echo.namespace().ok_or(ReconcileError::MissingMetadata)?;
    let deployments: Api<Deployment> =
        Api::namespaced(ctx.client.clone(), &namespace);
    let desired = desired_deployment(&echo)?;

    // Server-side apply: creates or updates, so reconciling twice is harmless.
    let applied = deployments
        .patch(
            &echo.name_any(),
            &PatchParams::apply("echo-operator"),
            &Patch::Apply(&desired),
        )
        .await?;

    // Update the status only when it changed, to avoid a reconcile loop.
    let ready = applied.status.and_then(|s| s.ready_replicas).unwrap_or(0);
    if echo.status.as_ref().map(|s| s.ready_replicas) != Some(ready) {
        let echoes: Api<Echo> = Api::namespaced(ctx.client.clone(), &namespace);
        echoes
            .patch_status(
                &echo.name_any(),
                &PatchParams::default(),
                &Patch::Merge(
                    json!({ "status": EchoStatus { ready_replicas: ready } }),
                ),
            )
            .await?;
    }
    Ok(Action::requeue(Duration::from_secs(300)))
}

/// Decides what to do after a failed reconciliation: retry later.
fn error_policy(
    _echo: Arc<Echo>,
    error: &ReconcileError,
    _ctx: Arc<Context>,
) -> Action {
    eprintln!("reconcile failed: {error}");
    Action::requeue(Duration::from_secs(15))
}

/// Runs the controller against the current cluster (kubeconfig).
async fn run_controller() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let echoes: Api<Echo> = Api::all(client.clone());
    let deployments: Api<Deployment> = Api::all(client.clone());
    Controller::new(echoes, Config::default())
        // Also reconcile when an owned Deployment changes:
        .owns(deployments, Config::default())
        .run(reconcile, error_policy, Arc::new(Context { client }))
        .for_each(|result| async move {
            match result {
                Ok((object, _action)) => println!("reconciled {}", object.name),
                Err(e) => eprintln!("controller error: {e}"),
            }
        })
        .await;
    Ok(())
}

/// Prints the CRD, which can be piped to `kubectl apply -f -`.
fn main() -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(&Echo::crd())?);
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn echo() -> Echo {
        let mut echo = Echo::new(
            "hello",
            EchoSpec {
                message: "hi".to_owned(),
                replicas: 2,
            },
        );
        echo.metadata.namespace = Some("default".to_owned());
        echo.metadata.uid = Some("1234".to_owned());
        echo
    }

    /// Runs the kubernetes example through its entry point.
    #[test]
    fn test_kubernetes_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn crd_has_expected_identity() {
        let crd = Echo::crd();
        assert_eq!(crd.spec.group, "example.com");
        assert_eq!(crd.spec.names.kind, "Echo");
        assert!(crd.spec.versions[0].subresources.is_some()); // status
    }

    #[test]
    fn desired_deployment_follows_the_spec() -> Result<(), ReconcileError> {
        let deployment = desired_deployment(&echo())?;
        let spec = deployment.spec.expect("spec");
        assert_eq!(spec.replicas, Some(2));
        let owners = deployment.metadata.owner_references.expect("owner");
        assert_eq!(owners[0].kind, "Echo");
        Ok(())
    }

    #[test]
    fn missing_uid_is_an_error() {
        let mut echo = echo();
        echo.metadata.uid = None;
        assert!(matches!(
            desired_deployment(&echo),
            Err(ReconcileError::MissingMetadata)
        ));
    }

    #[tokio::test]
    #[ignore = "needs a cluster (kind or k3d)"]
    async fn controller_starts() {
        run_controller().await.expect("cluster available");
    }
}
