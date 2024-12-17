# Implement Rust applications on AWS

{{#include aws.incl.md}}

[AWS rust][blog-sustainability-with-rust]⮳

At AWS, Rust has quickly become critical to building infrastructure at scale. `Firecracker` is an open source virtualization technology that powers AWS Lambda and other serverless offerings.

## Develop and deploy applications with the AWS SDK for Rust {#aws-sdk-rust}

Call AWS services using idiomatic Rust APIs.

[AWS Rust SDK][aws-rust-sdk-website]{{hi:AWS}}⮳

[AWS SDK examples][aws-doc-sdk-examples-github]⮳

```rust,editable
{{#include ../../../deps/tests/other/cloud/aws_sdk.rs:example}}
```

## Implement Lambda functions in Rust {#lambda}

[AWS Rust SDK][aws-rust-sdk-website]{{hi:AWS}}⮳

Use the AWS SDK from within a Lambda function:

[Create Lambda functions with Rust][aws-create-lambda-functions-with-rust]⮳

[AWS Lambda Rust runtime examples][aws-lambda-rust-runtime-examples-github]⮳

```rust,editable
{{#include ../../../deps/tests/other/cloud/aws_lambda.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[aws: write (P1)](https://github.com/john-cd/rust_howto/issues/574)

</div>
