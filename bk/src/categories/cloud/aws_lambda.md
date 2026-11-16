<!-- TODO Reviewed ??-??-2026. -->

# Test an AWS Lambda Handler Locally

Keep the event-to-response handler separate from the Lambda runtime entry point. This handler accepts a JSON object with a `name` and returns a greeting. Its unit test constructs a `LambdaEvent` directly, so it requires no AWS account.

```rust
{{#include ../../../crates/cats/cloud_lambda/src/main.rs:example}}
```

From `bk/crates`, run `cargo test -p cloud_lambda`. To exercise the runtime locally, open a terminal in `bk/crates/cats/cloud_lambda` and run `cargo lambda watch`; in another terminal there, run `cargo lambda invoke cloud_lambda --data-file fixtures/greet.json`. The expected response is `{"message":"Hello, Ada!"}`. Stop the watcher with Ctrl+C. Cargo Lambda's [getting-started guide](https://www.cargo-lambda.info/guide/getting-started.html) describes this emulator. It builds a Linux binary for the selected Lambda architecture; choose `x86_64` or `arm64` to match the deployment target, and ensure the corresponding toolchain is installed.

For an optional live deployment, supply AWS credentials and region through a profile or workload identity, configure an execution role with only the permissions the function needs, then run `cargo lambda build --release` and `cargo lambda deploy`. Record the function name and region and delete the function after the exercise to avoid ongoing costs. Keep secrets in environment configuration or a secret store. The [AWS Rust Lambda walkthrough](https://aws.amazon.com/blogs/compute/building-serverless-applications-with-rust-on-aws-lambda/) covers deployment details. See the [larger AWS draft](https://github.com/john-cd/rust_howto/blob/develop/later/src/other/cloud/aws.md) for SDK integration.
