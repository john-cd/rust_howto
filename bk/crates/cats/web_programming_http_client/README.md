<!-- TODO Reviewed ??-??-2026 -->

# Web Programming HTTP Client

This crate contains code examples for the **Web Programming HTTP Client** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `backon`, `clap`, `futures-util`, `governor`, `http-body-util`.

## Examples Covered

- **APIS** (`examples/apis/`): Demonstrates concepts like `paginated`, `rate_limited`, `rest_get`, `rest_head`, `rest_post`.
- **Download** (`examples/download/`): Demonstrates concepts like `download`, `partial`, `post_file`.
- **HTTP Clients** (`examples/http_clients/`): Demonstrates concepts like `hyper`, `ureq`.
- **Rate Limit** (`examples/rate_limit/`): Code examples covering rate limit.
- **Requests** (`examples/requests/`): Demonstrates concepts like `get`, `get1`, `header`.
- **Retry** (`examples/retry/`): Code examples covering retry.
- **Retry After** (`examples/retry_after/`): Code examples covering retry after.

## Usage

To check or run examples in this crate:

```console
cargo check --package web_programming_http_client --examples
cargo test --package web_programming_http_client
```

Code examples correspond to chapters in `bk/src/categories/web_programming_http_client` or related book sections.

## Running the File Upload Example

The `download` example's `post_file` command reads bytes from `temp/message`, relative to the process's current directory. Create that file before running `cargo run --example download -- post_file` from this package directory. The command sends the file to `https://httpbin.org/post`, so it requires network access. File read failures, request failures, and unsuccessful HTTP status codes are returned to the caller.

## Running the REST POST Example

The `apis rest_post` command creates a **public** GitHub Gist and then deletes it. If deletion fails, the created gist may remain public; the command prints its URL. Use a fine-grained personal access token with the **Gists** user permission set to **write**. Store the token in `GH_TOKEN` without adding it to source control, then run `cargo run --example apis -- rest_post` from this package directory. The example needs access to GitHub's REST API, so it is not suitable for the Rust Playground. The automated tests use a local HTTP server and need no token or external network access.
