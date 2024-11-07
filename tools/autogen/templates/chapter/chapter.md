# {{ section }}

{{#include index.incl.md}}

## {{ example }}

[![{{ crate_name }}][c-{{ crate_name }}-badge]][c-{{ crate_name }}]  [![cat-{{ category_slug }}][cat-{{ category_slug }}-badge]][cat-{{ category_slug }}]

$description1$

```rust
{% raw %}{{{% endraw %}#include ../../../deps/tests/{{ crate_name }}.rs:example{% raw %}}}{% endraw %}
```

{% raw %}
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
{% endraw %}
<div class="hidden">
</div>
