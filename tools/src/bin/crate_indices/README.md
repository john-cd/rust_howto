# Index of crates

## 'crates by category' page

To generate `crates_by_category.md`,

Extract all crates listed in `crates.md`

```sh
grep -Po '(?<=\]\[c-)\w+?(?=\])' ./src/key_crates/crates_alpabetical.md > ./tools/src/bin/crate_indices/crates.txt
```

`(?<= )` and `(?= )` are non-capturing look-behind and look-ahead groups, respectively.
`-P` for Perl regular expressions.
`-o` to return only the matching part, not the whole line.

Optionally, extract all crates currently used in the book examples from `deps/Cargo.toml`

```sh
just crate_indices l >> ./tools/src/bin/crate_indices/crates.txt
sort -u ./tools/src/bin/crate_indices/crates.txt > ./tools/src/bin/crate_indices/crates.txt
```

Then call the tool:

```sh
cat ./tools/src/bin/crate_indices/crates.txt | just crate_indices c > output.md
```

`just` calls `cargo run -p rust_howto_tools --bin crate_indices --`

You can merge with the existing table in `crates_by_category.md` using `sort -u temp.md > temp2.md`

## Alpahabetical list of crates used in the book

```sh
cat ./tools/src/bin/crate_indices/crates.txt | just crate_indices a > crates_alphabetical.md
```
