# Index of crates

## 'crates by category' page

To generate `crates_by_category.md`,

- extract all crates listed in `crates.md`

```sh
grep -Po '(?<=\]\[c-)\w+?(?=\])' ./src/key_crates/crates_alpabetical.md > ./extras/tools/crate_indices/crates.txt
```

`(?<= )` and `(?= )` are non-capturing look-behind and look-ahead groups, respectively.
`-P` for Perl regular expressions.
`-o` to return only the matching part, not the whole line.

- or extract all crates currently used in the book examples from `deps/Cargo.toml`

- `cat ./extras/tools/crate_indices/crates.txt | just templ c`

## Alpahabetical list of crates used in the book
