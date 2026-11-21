<!-- TODO Reviewed ??-??-2026. -->

# book-audit

`book-audit` checks rendered book output and visible source content. Run commands
from the repository root and pass `--repo-root` when the binary is launched from
another directory.

```sh
cargo run --locked -p book-audit -- --repo-root . rendered-links bk/book/html --all-chapters
cargo run --locked -p book-audit -- --repo-root . word-index bk/book/html
cargo run --locked -p book-audit -- --repo-root . playground bk/book/html --book-root bk
cargo run --locked -p book-audit -- --repo-root . search-index --book-root bk
cargo run --locked -p book-audit -- --repo-root . preview-links bk/book/future
cargo run --locked -p book-audit -- --repo-root . visible-todos
```

`just --justfile bk/justfile book buildbook` runs the published-book audits after
rendering. `just --justfile bk/justfile book future` renders and checks the staged
preview. The preview audit skips deferred Aerospace and Cryptocurrencies pages by
default; pass `--include-deferred` to the `preview-links` command to include them.
Findings include rendered paths and line numbers when available, and return a
failure exit status.
