# Static Website Generators

{{#include static_website_generators.incl.md}}

## Create a Simple Website Using a Static Website Generator {#zola}

[![zola][zola~website~badge]][zola~website] [![zola~github][zola~github~badge]][zola~github] [![zola~lib.rs][zola~lib.rs~badge]][zola~lib.rs]{{hi:zola}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

[Zola][zola~website]{{hi:zola}}↗ is a fast static site generator in a single binary with everything built-in.

Once Zola is installed, create a new site:

```sh
cargo install zola
zola init my_site
cd my_site
```

Open the `config.toml` file. Here's a basic example:

```toml
base_url = "https://example.com"
title = "My Zola Site"
description = "A simple site powered by Zola"
# Other Configuration Options...
```

Create a first post in `content/blog/first-post.md` with the following content:

```md
+++
title = "First Post"
date = 2025-01-10
+++

This is my first blog post using Zola!
```

```sh
zola build
zola serve
```

You can further customize your site by editing the templates in the templates directory and adding your styles in the static directory.

### Themes for `zola` {#skip1}

[AdiDoks][adidoks~website]↗ is a modern documentation theme.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[static_website_generators: write](https://github.com/john-cd/rust_howto/issues/519)
</div>
