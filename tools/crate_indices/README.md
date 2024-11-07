# Index of crates

The `crate_indices` tool creates indexes of crates e.g. by category and by alphabetical order.
Run the tool via `just crate_indices`.

## 'crates by category' page

To generate `crates_by_category.md`,

a. Extract all crates listed in `crates.md`

```sh
grep -Po '(?<=\]\[c-)\w+?(?=\])' ./src/key_crates/crates_alpabetical.md > ./crates/crate_indices/crates.txt
```

`(?<= )` and `(?= )` are non-capturing look-behind and look-ahead groups, respectively.
`-P` for Perl regular expressions.
`-o` to return only the matching part, not the whole line.

b. Optionally, extract all crates currently used in the book examples from `deps/Cargo.toml`

```sh
just crate_indices l >> ./crates/crate_indices/crates.txt
sort -u ./crates/crate_indices/crates.txt > ./crates/crate_indices/crates.txt
```

c. Call the tool:

```sh
cat ./crates/crate_indices/crates.txt | just crate_indices c > output.md
```

`just` calls `cargo run -p crate_indices --`

You can merge with the existing table in `crates_by_category.md` using `sort -u temp.md > temp2.md`

## Alphabetical list of crates used in the book

To generate the alphabetical index of crates, update `crates.txt` then use

```sh
cat ./crates/crate_indices/crates.txt | just crate_indices a > crates_alphabetical.md
```
