#!/usr/bin/env bash
# Purpose: Insert crate links.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
# TODO Reviewed ??-??-2026

set -euo pipefail

# Insert crate links
#
# Usage <script>.sh <root folder of book>
# Beware: modifies a lot of files. Manual editing is required after run.
# Fenced code blocks, headings, and existing reference-style links are left alone.

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <root folder of book>" >&2
  exit 2
fi

root="$(realpath -- "$1")/"
if [[ ! -d "$root/src" || ! -d "$root/drafts" ]]; then
  echo "Error: '$root' must contain src/ and drafts/ directories." >&2
  exit 1
fi

# Markdown files
suggest_file="$root/suggest.md"
touch "$suggest_file"

markdown_without_fenced_code() {
  awk '
    {
      line = $0
      sub(/^[[:space:]]+/, "", line)
      marker = substr(line, 1, 3)
      if (!in_fence && (marker == "```" || marker == "~~~")) {
        fence_character = substr(line, 1, 1)
        fence_length = 0
        while (substr(line, fence_length + 1, 1) == fence_character) fence_length++
        in_fence = 1
        next
      }
      if (in_fence) {
        closes = length(line) >= fence_length
        for (i = 1; closes && i <= fence_length; i++) {
          if (substr(line, i, 1) != fence_character) closes = 0
        }
        if (closes && substr(line, fence_length + 1) !~ /[^[:space:]]/) in_fence = 0
        next
      }
      print $0
    }
  ' "$1"
}

# Insert potential links to crates into the book's Markdown
while IFS= read -r -d '' file; do
  echo ">> ${file}"
  # Search the Markdown for `word` that is not in a link (i.e. not surrounded by [ ]) and not ```-quoted.
  # These are potential crate names.
  # -N = --no-line-number -I = --no-filename -r = replace -i = ignore case
  while IFS= read -r in_backticks; do
    if [[ ${#in_backticks} -gt 1 ]]; then
      trimmed="${in_backticks/%[\._-]rs/}" # remove .rs or -rs or _rs
      if [ "$trimmed" != "$in_backticks" ]; then
        pattern="${in_backticks}|${trimmed}"
      else
        pattern="${in_backticks}"
      fi
      underscored=$(printf '%s' "$pattern" | tr '-' '_')
      if [ "$underscored" != "$pattern" ]; then
        pattern="(${pattern}|${underscored})"
      else
        pattern="(${pattern})"
      fi
      echo "Pattern: ${pattern}"
      # Find the reference, if any, and create the link(s), separated by spaces if multiple. Trim last space.
      # -I = no filename; -N = no line number; -o = only-matching; -i = ignore case
      links=$(rg -INo --null-data '\[c~'"${pattern}"'~docs\]:' -r '[`'"${in_backticks}"'`][c~$1~docs]↗{{hi:'"${in_backticks}"'}}' "$root/src/refs/crate-refs.md" | tr '\0' ' ' | sed -E -e 's/ +$//' || true)
      if [ -n "${links}" ]; then
        echo "Links: ${links}"
        CRATE_NAME="$in_backticks" LINKS="$links" perl -0pi -e '
          my $crate = $ENV{"CRATE_NAME"};
          my $replacement = $ENV{"LINKS"};
          my ($fence_character, $fence_length) = ("", 0);
          my @lines = split /(?<=\n)/, $_;
          for my $line (@lines) {
            if ($fence_character ne "") {
              my $closing = qr/^\s*\Q$fence_character\E{$fence_length,}\s*$/;
              $fence_character = "" if $line =~ $closing;
              next;
            }
            if ($line =~ /^\s*(`{3,}|~{3,})/) {
              $fence_character = substr($1, 0, 1);
              $fence_length = length($1);
              next;
            }
            next if $line =~ /^\s*#/;
            $line =~ s{`\Q$crate\E`(?![\]`])}{
              my $matched = $&;
              my $prefix = substr($line, 0, $-[0]);
              $prefix =~ /\[[^\]]*$/ ? $matched : $replacement;
            }ge;
          }
          $_ = join "", @lines;
        ' "${file}"
      else
        # Add potentially missing crates to a file.
        echo "$in_backticks" | sed -E -e 's/<T>//g' >> "$suggest_file"
      fi
    fi
  done < <(markdown_without_fenced_code "$file" | rg -INioP '(?<![\[`])`([^ `\[\]]+?)`(?![\]`])' -r '$1' | sort -u || true)
done < <(find "$root/src" "$root/drafts" -type f \( -name "*.md" -not -name "SUMMARY.md" -not -name "examples_index.md" -not -name "*.incl.md" \) -print0)

# Sort and make unique
sort -u -o "$suggest_file" "$suggest_file"

echo "DONE"
