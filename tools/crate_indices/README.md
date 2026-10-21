<!-- TODO Reviewed ??-??-2026 -->
# Index of crates

The `crate_indices` tool creates indexes of crates e.g. by category and by alphabetical order.
Run the tool via `just indices crate_indices`.

## Update the master list of crates

a. Extract all crates currently used in the book examples from `crates/**/Cargo.toml`

```sh
cd bk
just indices crate_indices list_crates -d . >> ./master/crates.txt
sort -u -o ./master/crates.txt ./master/crates.txt
```

b. Optionally extract all crates listed in `crates_alphabetical.md`

```sh
cd bk
grep -Po '(?<=\]\[c~)\w+?(?=~docs\])' ./src/indices/crates_alphabetical.md >> ./master/crates.txt
```

`(?<= )` and `(?= )` are non-capturing look-behind and look-ahead groups, respectively.
`-P` for Perl regular expressions.
`-o` to return only the matching part, not the whole line.

c. Check the list of crates against the crates.io registry:

```sh
just crates check
```

## Generate the 'crates by category' page

`bk/master/crate_categories.toml` is the checked-in snapshot of external crates.io
category metadata. Regenerate the published page from that snapshot, the book's crate
inventory, and the crates found in published recipes without network access. Each
crate links to its published recipes:

```sh
cd bk
just book _generate-index-category
```

The `category_page` command remains available to query crates.io when refreshing
the metadata snapshot. Update the snapshot deliberately, then rerun the offline
generator and review the resulting diff.

## Generate the alphabetical list of crates used in the book

The alphabetical index also discovers crate badges in the published recipe tables
and links each crate to every matching recipe anchor. Generate it from the book root:

```sh
cd bk
just book _generate-index-alphabetical
```

The generator follows the include fragments listed by `src/indices/examples_index.md`,
checks that each recipe reference is defined, and sorts crate and recipe links for
repeatable output.

## Update the master file of reference definitions

All URLs to crates.io, docs.rs. lib.rs... for the book are stored in `./src/refs/crate-refs.md`.

Update the master file of reference definition using the following command:

```sh
cat ./master/crates.txt | just indices crate_indices update_refdefs -f ./src/refs/crate-refs.md
```

Edit by hand where needed.
