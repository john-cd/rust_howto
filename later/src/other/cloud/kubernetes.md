<!-- TODO Reviewed ??-??-2026. -->

# Kubernetes Operators

{{#include kubernetes.incl.md}}

An operator is a program that watches custom resources and keeps the cluster in the state they describe. Rust is a good fit: the type system models the resource, and the controller runtime handles watching, caching and retries.

## Write an Operator with `kube-rs` {#kubernetes}

{{#badges kube development-tools="Development tools"}}

[`kube`][c~kube~docs]↗{{hi:kube}} provides a Kubernetes client, a derive macro for custom resources (the CRD is generated from the Rust types, using [`schemars`][c~schemars~docs]↗{{hi:schemars}}), and a controller runtime. [`k8s-openapi`][c~k8s-openapi~docs]↗{{hi:k8s-openapi}} supplies the built-in types such as `Deployment`.

The recipe has three layers:

1. The `Echo` custom resource and its CRD. The example prints the CRD as
   JSON and does not connect to a cluster.
2. A pure function from the spec to the desired `Deployment`, unit-tested without a cluster.
3. The `Controller` loop with `reconcile` and `error_policy`. Its test is `#[ignore]`d because it needs a cluster ([kind](https://kind.sigs.k8s.io/)↗ or [k3d](https://k3d.io/)↗).

From the repository root, generate the CRD with this command.

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p other --example cloud -- kubernetes
```

Check that `kind` is `CustomResourceDefinition` and that `metadata.name`
is `echos.example.com`. Inspect this output
before applying it. For cluster validation, use a disposable kind or k3d
cluster and explicitly select its kubectl context. The CRD is cluster-wide
even though each `Echo` resource is namespaced. The ignored controller test
starts a long-running watcher. Confirm a test resource produces a Deployment
and updated status, then stop the watcher and tear down the disposable cluster.
Generating the CRD and checking the desired Deployment offline establish
different behavior from an actual reconciliation.

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
