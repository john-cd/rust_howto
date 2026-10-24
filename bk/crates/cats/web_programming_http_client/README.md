<!-- TODO Reviewed ??-??-2026 -->

# README

Code examples are found under `examples/<chapter_name>`, where `<chapter_name>` is the name of the `.md` file in `bk/<...>/categories/<category_name>` that includes these examples.

## Running the File Upload Example

The `download` example's `post_file` command reads bytes from `temp/message`, relative to the process's current directory. Create that file before running `cargo run --example download -- post_file` from this package directory. The command sends the file to `https://httpbin.org/post`, so it requires network access. File read failures, request failures, and unsuccessful HTTP status codes are returned to the caller.

## Running the REST POST Example

The `apis rest_post` command creates a **public** GitHub Gist and then deletes it. If deletion fails, the created gist may remain public; the command prints its URL. Use a fine-grained personal access token with the **Gists** user permission set to **write**. Store the token in `GH_TOKEN` without adding it to source control, then run `cargo run --example apis -- rest_post` from this package directory. The example needs access to GitHub's REST API, so it is not suitable for the Rust Playground. The automated tests use a local HTTP server and need no token or external network access.
