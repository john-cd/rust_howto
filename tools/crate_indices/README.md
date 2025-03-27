# Index of crates

The `crate_indices` tool creates indexes of crates e.g. by category and by alphabetical order.
Run the tool via `just crate_indices`.

## Update the master list of crates

a. Extract all crates listed in `crates.md`

```sh
cd bk
grep -Po '(?<=\]\[c-)\w+?(?=\])' ./src/crates/crates_alphabetical.md >> ./master/crates.txt
```

`(?<= )` and `(?= )` are non-capturing look-behind and look-ahead groups, respectively.
`-P` for Perl regular expressions.
`-o` to return only the matching part, not the whole line.

b. Optionally, extract all crates currently used in the book examples from `crates/**/Cargo.toml`

```sh
cd bk
just indices crate_indices list_crates -d . >> ./master/crates.txt
sort -u -o ./master/crates.txt ./master/crates.txt
```

## 'crates by category' page

To generate `crates_by_category.md`, call the tool:

```sh
cd bk
cat ./master/crates.txt | just indices crate_indices category_page > output.md
```

`just` calls `../bin/crate-indices`

You can merge with the existing table in `crates_by_category.md` using `sort -u -o file.md file.md`

## Alphabetical list of crates used in the book

To generate the alphabetical index of crates, update `crates.txt` then use

```sh
cd bk
cat ./master/crates.txt | just indices crate_indices alphabetical_page > crates_alphabetical.md
```

## Master file of reference definitions

All URLs to crates.io, docs.rs. lib.rs... for the book are stored in `./src/refs/crate-refs.md`.

Update the master file of reference definition using the following command:

```sh
cat ./master/crates.txt | just indices crate_indices update_refdefs -f ./src/refs/crate-refs.md
```

Edit by hand where needed.
