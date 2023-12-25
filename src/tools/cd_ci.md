# Continuous Deployment / Continuous Integration

[Continuous Integration (cargo book)]( https://doc.rust-lang.org/cargo/guide/continuous-integration.html )

## GitHub Actions

[install-action]( https://github.com/taiki-e/install-action ) is a GitHub Action for installing development tools (mainly from GitHub Releases).

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: cargo-binstall,just,mdbook,mdbook-lintcheck
```

## See also

[Optimizing CI/CD pipelines in your Rust projects]( https://blog.logrocket.com/optimizing-ci-cd-pipelines-rust-projects/ )
