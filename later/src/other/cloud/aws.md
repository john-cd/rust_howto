<!-- TODO Reviewed ??-??-2026 -->

# Implement Rust Applications on AWS

{{#include aws.incl.md}}

Amazon Web Services (AWS) is a comprehensive and widely adopted cloud computing platform provided by Amazon.com.

At AWS, Rust has quickly become critical to building infrastructure at scale. For example, `Firecracker`, written in majority in Rust, is an open source [virtualization][p~virtualization] technology that powers AWS Lambda and other serverless offerings.

In addition, Rust is commonly used by AWS' clients to build cloud applications, from backends to complete web applications.

Rust code is frequently implemented as:

- serverless functions that run on a Function-as-a-Service (FaaS) platform like the aforementioned AWS Lambda,
- (micro)services that are then deployed on a containerized platform (AWS ECS, EKS, App Runner...) or on EC2 machines.

## Implement AWS Lambda Functions in Rust {#lambda}

Rust's performance and small binary size make it a great choice for serverless functions. Rust Lambda functions can start quickly and consume fewer resources, leading to lower costs and improved performance.

The following example provides a testable Lambda handler written in Rust. Install [Cargo Lambda][cargo-lambda~website]↗ to build and deploy it, then use `cargo lambda watch` to run it locally. Before deploying, configure AWS credentials and a deployment role.

You can also use the AWS SDK from within a Lambda function to interact with AWS services like S3 or CloudWatch (see the example below).

```rust,editable
{{#include ../../../crates/other/examples/cloud/aws_lambda.rs:example}}
```

See also the ["Create Lambda Functions with Rust"][aws-create-lambda-functions-with-rust~website]↗ and ["AWS Lambda Rust Runtime Examples"][aws-lambda-rust-runtime~examples~repo]↗ webpages.

## Interact with AWS Services with the AWS SDK for Rust {#aws-sdk-rust}

AWS has developed an official AWS SDK for Rust, which allows developers to interact with AWS services using the Rust programming language.
This SDK provides a set of libraries that simplify the use of services like Amazon S3, Amazon EC2, DynamoDB, and others.

See, for example:

- [AWS Rust SDK][aws-rust-sdk~website]↗{{hi:AWS}}.
- [AWS SDK examples][aws-doc-sdk~examples~repo]↗.

```rust,editable
{{#include ../../../crates/other/examples/cloud/aws_sdk.rs:example}}
```

## Share the AWS Configuration Across Clients {#aws-shared-config}

[![aws-config][c~aws-config~docs~badge]][c~aws-config~docs]{{hi:aws-config}}

Load the `SdkConfig` once, then build every service client from it. The default credential provider chain checks environment variables, shared profiles, SSO, and container or instance roles, so code never contains keys. Pin a `BehaviorVersion`, and set retries and timeouts explicitly.

```rust,no_run
{{#include ../../../crates/other/examples/cloud/aws_shared_config.rs:example}}
```

## Send Email with Amazon SES {#aws-ses}

[![aws-sdk-sesv2][c~aws-sdk-sesv2~docs~badge]][c~aws-sdk-sesv2~docs]{{hi:aws-sdk-sesv2}}

Send a multipart message with an HTML body and a plain-text fallback. The sender must be a verified identity. Request construction is separated from sending so the unit test needs no AWS account.

```rust,no_run
{{#include ../../../crates/other/examples/cloud/aws_ses.rs:example}}
```

## Publish Metrics to Amazon CloudWatch {#aws-cloudwatch}

[![aws-sdk-cloudwatch][c~aws-sdk-cloudwatch~docs~badge]][c~aws-sdk-cloudwatch~docs]{{hi:aws-sdk-cloudwatch}}

Send custom metrics with dimensions. Every unique combination of dimension values is billed as a separate metric, so avoid unbounded values such as user identifiers.

```rust,no_run
{{#include ../../../crates/other/examples/cloud/aws_cloudwatch.rs:example}}
```

## Invoke an AWS Lambda Function {#aws-lambda-invoke}

[![aws-sdk-lambda][c~aws-sdk-lambda~docs~badge]][c~aws-sdk-lambda~docs]{{hi:aws-sdk-lambda}}

Invoke a function synchronously (`RequestResponse`) and parse its JSON payload, or fire and forget with `InvocationType::Event`. A failing function still returns HTTP 200: check `function_error`. See the function example [above](#lambda) for the handler.

```rust,no_run
{{#include ../../../crates/other/examples/cloud/aws_lambda_invoke.rs:example}}
```

## Use an S3-compatible Store {#aws-s3-compatible}

[![aws-sdk-s3][c~aws-sdk-s3~docs~badge]][c~aws-sdk-s3~docs]{{hi:aws-sdk-s3}}

MinIO, RustFS and LocalStack speak the S3 protocol. Point the client at the store's endpoint, supply static development credentials, and enable path-style addressing. The integration test is `#[ignore]`d unless a server is running.

```rust,no_run
{{#include ../../../crates/other/examples/cloud/aws_s3_compatible.rs:example}}
```

## Exchange Typed Messages over AWS SQS and SNS {#aws-message-bus}

[![aws-sdk-sqs][c~aws-sdk-sqs~docs~badge]][c~aws-sdk-sqs~docs]{{hi:aws-sdk-sqs}} [![aws-sdk-sns][c~aws-sdk-sns~docs~badge]][c~aws-sdk-sns~docs]{{hi:aws-sdk-sns}} [![prost][c~prost~docs~badge]][c~prost~docs]{{hi:prost}}

Hide the queue behind `MessagePublisher` and `MessageSubscriber` traits so business code and tests use an in-memory implementation, while production uses SQS (point-to-point) or SNS (fan-out to several queues). Payloads are Protobuf messages encoded with `prost` and base64, within the 256 KiB SQS limit. SQS delivers at least once: acknowledge (delete) a message only after it is handled, make handlers idempotent, tune the visibility timeout, and configure a dead-letter queue for messages that keep failing.

```rust,editable
{{#include ../../../crates/other/examples/cloud/aws_message_bus.rs:example}}
```

## References {#references .skip}

- [AWS and Rust][blog~sustainability-with-rust]↗.
- [AWS Rust SDK][aws-rust-sdk~website]↗{{hi:AWS}}.
- [Serverless Rust][serverless-rust~website]↗{{hi:Serverless}} framework.

## Related Topics {#related-topics .skip}

- [[cloud]]
- [[container]]
- [[microservices]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[aws: write](https://github.com/john-cd/rust_howto/issues/574)
Cover?
Containerized Applications on AWS Fargate, ECS, EKS
Deploy Rust Applications to Elastic Beanstalk
Deploy Rust Applications to App Runner
Deploy Rust Applications to EC2 machines

- [`aws-config`][c~aws-config~lib.rs]↗.

</div>
