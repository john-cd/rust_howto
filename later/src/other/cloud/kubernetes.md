<!-- TODO Reviewed ??-??-2026. -->

# Kubernetes Operators

{{#include kubernetes.incl.md}}

An operator watches custom resources and reconciles the cluster with the state they describe. In this recipe, setting `spec.replicas` on an `Echo` requests that many replicas in an owned Deployment. The operator reports the number that are ready in the resource's status.

## Write an Operator with `kube-rs` {#kubernetes}

{{#badges kube development-tools="Development tools"}}

[`kube`][c~kube~docs]↗{{hi:kube}} provides a Kubernetes client, a derive macro for custom resources (the CRD is generated from the Rust types, using [`schemars`][c~schemars~docs]↗{{hi:schemars}}), and a controller runtime. [`k8s-openapi`][c~k8s-openapi~docs]↗{{hi:k8s-openapi}} supplies the built-in types such as `Deployment`.

The recipe has three layers:

1. The `Echo` custom resource and its CRD. The example prints the CRD as
   JSON and does not connect to a cluster.
2. A pure function from the spec to the desired `Deployment`, unit-tested without a cluster.
3. The `Controller` loop with `reconcile` and `error_policy`. It runs only when
   you supply `--kubeconfig`.

From the repository root, generate the CRD with this command.

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p other --example cloud -- kubernetes
```

Check that `kind` is `CustomResourceDefinition` and that `metadata.name`
is `echos.example.com`.

## Verify Reconciliation in a Disposable Cluster

Use [kind](https://kind.sigs.k8s.io/) with a running Docker engine and `kubectl`.
From the repository root, create a test cluster and direct each command to its
own kubeconfig file. The CRD is cluster-wide even though each `Echo` is namespaced. Use Cargo directly when redirecting the CRD so the output contains only JSON. For this repository on Windows, use the checkout's configured Cargo target and build paths.

```sh
kind create cluster --name howto-echo --kubeconfig echo-kubeconfig.yaml
cargo run --quiet --manifest-path later/crates/Cargo.toml --locked -p other --example cloud -- kubernetes > echo-crd.json
kubectl --kubeconfig echo-kubeconfig.yaml apply -f echo-crd.json
kubectl --kubeconfig echo-kubeconfig.yaml wait --for=condition=Established crd/echos.example.com --timeout=45s
just cargo run --manifest-path later/crates/Cargo.toml --locked -p other --example cloud -- kubernetes --kubeconfig echo-kubeconfig.yaml
```

With the controller running, save the following as `echo.yaml`.

```yaml
apiVersion: example.com/v1
kind: Echo
metadata:
  name: staged-echo
  namespace: default
spec:
  message: hello from staged Kubernetes
  replicas: 1
```

In a second terminal, apply it and wait for the controller to report a ready replica.
Deleting the `Echo` should also remove the Deployment it owns.

```sh
kubectl --kubeconfig echo-kubeconfig.yaml apply -f echo.yaml
kubectl --kubeconfig echo-kubeconfig.yaml wait '--for=jsonpath={.status.ready_replicas}=1' echo/staged-echo --timeout=120s
kubectl --kubeconfig echo-kubeconfig.yaml delete echo staged-echo --cascade=foreground --wait=true --timeout=45s
kubectl --kubeconfig echo-kubeconfig.yaml get deployment staged-echo --ignore-not-found
```

Foreground deletion waits for the owned Deployment to be removed. The final command should produce no output.
Stop the controller with Ctrl+C and
run `kind delete cluster --name howto-echo --kubeconfig echo-kubeconfig.yaml`.

## Controller Design

- **Reconciliation must be idempotent**: it runs repeatedly, and may run for events it has already handled. Server-side apply creates or updates the same object.
- **Owner references** let Kubernetes delete the children of a deleted resource. Use _finalizers_ when you must clean up something outside the cluster first.
- **Status subresource**: write observed state with `patch_status`, and skip the write when nothing changed, or you will trigger another reconcile.
- **Requeue strategy**: return `Action::requeue(..)` for periodic resync, and use `error_policy` to retry failures after a delay.

```rust,editable
{{#include ../../../crates/other/examples/cloud/kubernetes.rs:example}}
```

## Related Topics {.skip}

- [[cloud | Cloud]].
- [[virtualization | Virtualization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
