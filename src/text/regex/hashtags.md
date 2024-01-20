## Extract a list of unique #Hashtags from a text

[![regex-badge]][regex] [![lazy_static-badge]][lazy_static] [![cat-text-processing-badge]][cat-text-processing]

Extracts, sorts, and deduplicates list of hashtags from text.

The hashtag regex given here only catches Latin hashtags that start with a
letter. The complete [twitter hashtag regex] is much more complicated.

```rust,editable
{#include ../../../deps/examples/hashtags.rs}
```

[twitter hashtag regex]: https://github.com/twitter/twitter-text/blob/c9fc09782efe59af4ee82855768cfaf36273e170/java/src/com/twitter/Regex.java#L255
