#!/usr/bin/env bash
# Purpose: Insert links to categories: [<category>][cat~<category>].
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
# TODO Reviewed ??-??-2026

set -euo pipefail

# Print usage for the dry-run and apply modes.
usage() {
    printf 'Usage: %s <book-root> [--apply] [--include-drafts]\n' "$0" >&2
}

if [[ $# -lt 1 ]]; then
    usage
    exit 2
fi

book_root_arg=$1
shift
mode=--dry-run
include_drafts=false
while [[ $# -gt 0 ]]; do
    case "$1" in
        --apply) mode=--apply ;;
        --include-drafts) include_drafts=true ;;
        *) usage; exit 2 ;;
    esac
    shift
done
if [[ ! -d "$book_root_arg" ]]; then
    printf 'Book root does not exist: %s\n' "$book_root_arg" >&2
    exit 2
fi

book_root=$(cygpath -u "$(cd -- "$book_root_arg" && pwd -P)")
source_root="$book_root/src"
draft_root="$book_root/drafts"
category_refs="$source_root/refs/category-refs.md"
if [[ ! -d "$source_root" || ! -f "$category_refs" ]]; then
    printf 'Expected src/ and src/refs/category-refs.md under: %s\n' "$book_root" >&2
    exit 2
fi

tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/insert-category-links.XXXXXX")
trap 'rm -rf -- "$tmp_root"' EXIT

declare -A output_candidates=()
declare -A required_refs=()
declare -A existing_refs=()
declare -A category_pages=()
declare -A conflicting_categories=()
declare -A found_terms=()
declare -a categories=()
declare -a markdown_files=()

# Read only the category search reference definitions, preserving slugs with namespaces.
mapfile -t categories < <(
    sed -nE 's/^\[cat~([^]]+)\]:[[:space:]]*https:\/\/crates\.io\/categories\/[^[:space:]]+.*/\1/p' "$category_refs"
)
if [[ ${#categories[@]} -eq 0 ]]; then
    printf 'No crates.io category references found in %s\n' "$category_refs" >&2
    exit 1
fi
mapfile -t categories < <(printf '%s\n' "${categories[@]}" | awk '{ print length($0) "\t" $0 }' | sort -rn | cut -f2-)

# Build fixed-string search terms so unrelated Markdown pages skip the rewrite pass.
term_file="$tmp_root/category-terms.txt"
for category in "${categories[@]}"; do
    if [[ ${#category} -lt 3 ]]; then
        continue
    fi
    humanized=${category//::/ }
    humanized=${humanized//-/ }
    printf '%s\n' "$category" "$humanized"
done | sort -fu > "$term_file"

# Search published pages by default; include drafts only when requested.
find_roots=("$source_root")
if [[ "$include_drafts" == true && -d "$draft_root" ]]; then
    find_roots+=("$draft_root")
fi
matched_files="$tmp_root/matched-files"
if rg --files-with-matches --null --ignore-case --fixed-strings --file "$term_file" \
    --glob '*.md' \
    --glob '!SUMMARY.md' \
    --glob '!examples_index.md' \
    --glob '!word_index.md' \
    --glob '!crates_by_category.md' \
    --glob '!crates_alphabetical.md' \
    --glob '!*.incl.md' \
    --glob '!**/refs/**' \
    --glob '!**/crates/**' \
    "${find_roots[@]}" > "$matched_files"; then
    mapfile -d '' -t markdown_files < "$matched_files"
else
    search_status=$?
    if [[ $search_status -ne 1 ]]; then
        exit "$search_status"
    fi
fi
for index in "${!markdown_files[@]}"; do
    markdown_files[$index]=$(cygpath -u "${markdown_files[$index]}")
done

if [[ ${#markdown_files[@]} -eq 0 ]]; then
    printf 'No category mentions found under src/ or drafts/.\n'
    exit 0
fi

run_id="${RANDOM}${RANDOM}"
marker_prefix="__RHCAT_${run_id}_"
block_prefix="__RHBLOCK_${run_id}_"
declare -A marked_categories=()
for file in "${markdown_files[@]}"; do
    relative_file=${file#"$book_root"/}
    candidate="$tmp_root/output/$relative_file"
    mkdir -p -- "$(dirname -- "$candidate")"
    cp -- "$file" "$candidate"
    required_refs=()
    category_pages=()
    conflicting_categories=()
    found_terms=()
    reference_file="$(dirname -- "$file")/refs.incl.md"
    has_local_refs=false
    existing_refs=()

    mapfile -t matched_terms < <(
        rg --only-matching --no-filename --no-line-number --ignore-case --fixed-strings --file "$term_file" -- "$file" | sort -fu || true
    )
    for matched_term in "${matched_terms[@]}"; do
        found_terms["${matched_term,,}"]=''
    done

    if grep -Eq '\{\{#include[[:space:]]+refs\.incl\.md[[:space:]]*\}\}' "$file"; then
        has_local_refs=true
    fi
    if [[ "$has_local_refs" == true && -f "$reference_file" ]]; then
        while IFS= read -r reference_line; do
            if [[ "$reference_line" =~ ^\[p~([^]]+)\]:[[:space:]]*(.+)$ ]]; then
                reference_slug=${BASH_REMATCH[1]}
                reference_target=${BASH_REMATCH[2]}
                if [[ -n ${existing_refs["$reference_slug"]+set} && ${existing_refs["$reference_slug"]} != "$reference_target" ]]; then
                    existing_refs["$reference_slug"]='__CONFLICT__'
                else
                    existing_refs["$reference_slug"]=$reference_target
                fi
            fi
        done < "$reference_file"
    fi

    rules_file=$(mktemp "$tmp_root/rules.XXXXXX")
    cat > "$rules_file" <<'SED'
/^[[:space:]]*#/ b skip
/.*\{#[a-zA-Z0-9_-]+.*/ b skip
/^[[:space:]]*```/ , /^[[:space:]]*```/ b skip
/^[[:space:]]*~~~/ , /^[[:space:]]*~~~/ b skip
/\{\{.*\}\}/ b skip
/\[\[.*\]\]/ b skip
/\]\([^)]*\)/ b skip
/\]\[[^]]*\]/ b skip
/`.*`/ b skip
SED
    category_id=0
    for category in "${categories[@]}"; do
        if [[ ${#category} -lt 3 ]]; then
            continue
        fi
        if [[ ! "$category" =~ ^[a-z0-9-]+(::[a-z0-9-]+)*$ ]]; then
            printf 'Unsupported category slug in %s: %s\n' "$category_refs" "$category" >&2
            exit 1
        fi

        category_pattern=$category
        humanized=${category//::/ }
        humanized=${humanized//-/ }
        if [[ "$humanized" != "$category" ]]; then
            category_pattern+="|$humanized"
        fi
        found_category=false
        for search_term in "$category" "$humanized"; do
            normalized_term=${search_term,,}
            if [[ -n ${found_terms["$normalized_term"]+set} ]]; then
                found_category=true
                break
            fi
        done
        if [[ "$found_category" == false ]]; then
            continue
        fi

        category_folder=${category//::/_}
        category_page="$source_root/categories/$category_folder/index.md"
        if [[ ! -f "$category_page" && "$include_drafts" == true && "$file" == "$draft_root/"* ]]; then
            category_page="$draft_root/categories/$category_folder/index.md"
        fi
        if [[ -f "$category_page" && "$file" == "$category_page" ]]; then
            continue
        fi
        if [[ ! -f "$category_page" ]]; then
            category_page=''
        fi

        internal_link=false
        marked_categories["$category_id"]=$category
        if [[ -n "$category_page" && "$has_local_refs" == true ]]; then
            relative_page=$(realpath --relative-to="$(dirname -- "$file")" "$category_page")
            reference_slug="category-$category"
            existing_target=${existing_refs["$reference_slug"]-}
            if [[ -z "$existing_target" || "$existing_target" == "$relative_page" ]]; then
                internal_link=true
                category_pages["$category"]=$category_page
            else
                conflicting_categories["$category"]=''
            fi
        fi

        prefix='(^|[^a-zA-Z~/{#:\[-])'
        suffix='([^a-zA-Z._-]|\.[[:space:]]|\.$|$)'
        if [[ "$internal_link" == true ]]; then
            replacement="\\1[\\2$block_prefix${category_id}__][p~category-$category]{{hi:$category}}$marker_prefix${category_id}__\\3"
        else
            replacement="\\1[\\2$block_prefix${category_id}__][cat~$category]↗{{hi:$category}}\\3"
        fi
        printf 's@%s(%s)%s@%s@gI\n' "$prefix" "$category_pattern" "$suffix" "$replacement" >> "$rules_file"
        category_id=$((category_id + 1))
    done
    printf ':skip\n' >> "$rules_file"

    next_candidate=$(mktemp "$tmp_root/rewrite.XXXXXX")
    sed -E -f "$rules_file" "$candidate" > "$next_candidate"
    mapfile -t used_markers < <(grep -oE "${marker_prefix}[a-z0-9:-]+__" "$next_candidate" | sort -u || true)
    if [[ ${#used_markers[@]} -gt 0 ]]; then
        for marker in "${used_markers[@]}"; do
            category_id=${marker#"$marker_prefix"}
            category_id=${category_id%__}
            category=${marked_categories["$category_id"]}
            category_page=${category_pages["$category"]}
            relative_page=$(realpath --relative-to="$(dirname -- "$file")" "$category_page")
            reference_definition="[p~category-$category]: $relative_page"
            required_refs["$reference_file|$reference_definition"]=''
        done
    fi
    sed -E \
        -e "s@${block_prefix}[a-z0-9:-]+__@@g" \
        -e "s@${marker_prefix}[a-z0-9:-]+__@@g" \
        "$next_candidate" > "$candidate"

    for category in "${!conflicting_categories[@]}"; do
        if grep -Fq "[cat~$category]" "$candidate"; then
            printf 'Using crates.io fallback for %s in %s because its local page reference conflicts.\n' \
                "$category" "$file" >&2
        fi
    done

    if ! cmp -s -- "$file" "$candidate"; then
        output_candidates["$file"]=$candidate
    fi

    for reference in "${!required_refs[@]}"; do
        reference_file=${reference%%|*}
        reference_definition=${reference#*|}
        references_candidate=${output_candidates["$reference_file"]-}
        if [[ -z "$references_candidate" ]]; then
            relative_refs=${reference_file#"$book_root"/}
            references_candidate="$tmp_root/output/$relative_refs"
            mkdir -p -- "$(dirname -- "$references_candidate")"
            if [[ -f "$reference_file" ]]; then
                cp -- "$reference_file" "$references_candidate"
            else
                : > "$references_candidate"
            fi
            output_candidates["$reference_file"]=$references_candidate
        fi
        if ! grep -Fqx -- "$reference_definition" "$references_candidate"; then
            printf '%s\n' "$reference_definition" >> "$references_candidate"
        fi
    done
done

if [[ ${#output_candidates[@]} -eq 0 ]]; then
    printf 'No category links need updating.\n'
    exit 0
fi

mapfile -t output_files < <(printf '%s\n' "${!output_candidates[@]}" | sort)
if [[ "$mode" == --dry-run ]]; then
    printf 'Dry run only. Review this diff, then pass --apply to write changes.\n'
    for original in "${output_files[@]}"; do
        relative_file=${original#"$book_root"/}
        if [[ -f "$original" ]]; then
            if diff -u --label "a/$relative_file" --label "b/$relative_file" "$original" "${output_candidates[$original]}"; then
                :
            else
                diff_status=$?
                if [[ $diff_status -ne 1 ]]; then
                    exit "$diff_status"
                fi
            fi
        else
            if diff -u --label /dev/null --label "b/$relative_file" /dev/null "${output_candidates[$original]}"; then
                :
            else
                diff_status=$?
                if [[ $diff_status -ne 1 ]]; then
                    exit "$diff_status"
                fi
            fi
        fi
    done
    exit 0
fi

for original in "${output_files[@]}"; do
    original_directory=$(dirname -- "$original")
    original_name=$(basename -- "$original")
    replacement_file=$(mktemp "$original_directory/.$original_name.XXXXXX")
    if [[ -f "$original" ]]; then
        cp --preserve=mode -- "${output_candidates[$original]}" "$replacement_file"
    else
        cp -- "${output_candidates[$original]}" "$replacement_file"
    fi
    mv -f -- "$replacement_file" "$original"
    printf 'Updated %s\n' "${original#"$book_root"/}"
done
