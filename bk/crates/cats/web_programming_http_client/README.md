<!-- TODO Reviewed ??-??-2026 -->
# README

Code examples are found under `examples/<chapter_name>`, where `<chapter_name>` is the name of the `.md` file in `bk/<...>/categories/<category_name>` that includes these examples.

## Running the File Upload Example

The `download` example's `post_file` command reads bytes from `temp/message`, relative to the process's current directory. Create that file before running `cargo run --example download -- post_file` from this package directory. The command sends the file to `https://httpbin.org/post`, so it requires network access. File read failures, request failures, and unsuccessful HTTP status codes are returned to the caller.
