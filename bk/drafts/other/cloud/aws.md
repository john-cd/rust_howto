# Implement Rust Applications on AWS

{{#include aws.incl.md}}

Amazon Web Services (AWS) is a comprehensive and widely adopted cloud computing platform provided by Amazon.com.

At AWS, Rust has quickly become critical to building infrastructure at scale. For example, `Firecracker`, written in majority in Rust, is an open source [virtualization][p-virtualization] technology that powers AWS Lambda and other serverless offerings.

In addition, Rust is commonly used by AWS' clients to build cloud applications, from backends to complete web applications.

Rust code is frequently implemented as:

- serverless functions that run on a Function-as-a-Service (FaaS) platform like the aforementioned AWS Lambda,
- (micro)services that are then deployed on a containerized platform (AWS ECS, EKS, App Runner...) or on EC2 machines.

## Implement AWS Lambda Functions in Rust {#lambda}

Rust's performance and small binary size make it a great choice for serverless functions. Rust Lambda functions can start quickly and consume fewer resources, leading to lower costs and improved performance.

The following example provides a template for a Lambda function written in Rust.

You can also use the AWS SDK from within a Lambda function to interact with AWS services like S3 or CloudWatch (see the example below).

```rust,editable
{{#include ../../../crates/other/tests/cloud/aws_lambda.rs:example}}
```

See also the ["Create Lambda functions with Rust"][aws-create-lambda-functions-with-rust]⮳ and ["AWS Lambda Rust runtime examples"][aws-lambda-rust-runtime-examples-github]⮳ webpages.

## Interact with AWS Services with the AWS SDK for Rust {#aws-sdk-rust}

AWS has developed an official AWS SDK for Rust, which allows developers to interact with AWS services using the Rust programming language.
This SDK provides a set of libraries that simplify the use of services like Amazon S3, Amazon EC2, DynamoDB, and others.

See, for example:

- [AWS Rust SDK][aws-rust-sdk-website]{{hi:AWS}}⮳.
- [AWS SDK examples][aws-doc-sdk-examples-github]⮳.

```rust,editable
{{#include ../../../crates/other/tests/cloud/aws_sdk.rs:example}}
```

## References

- [AWS and Rust][blog-sustainability-with-rust]⮳.
- [AWS Rust SDK][aws-rust-sdk-website]{{hi:AWS}}⮳.
- [Serverless rust][serverless-rust-website]{{hi:Serverless}}⮳ framework.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[aws: write](https://github.com/john-cd/rust_howto/issues/574)
Cover?
## Containerized Applications on AWS Fargate, ECS, EKS
## Deploy Rust Applications to Elastic Beanstalk
## Deploy Rust Applications to App Runner
## Deploy Rust Applications to EC2 machines
</div>
