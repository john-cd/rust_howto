#!/usr/bin/env bash
set -uo pipefail

# Rough script to convert inline links e.g. [...](http://...) into reference-style links: [...][...] [...]: http://...
# - Match link with a refdef, replace the link by the refdef label, if possible.
# - If there is no match, try to extract crate name from the link and add to refdef file
# - Otherwise, print a possible refdef

root="$(realpath $1)/"

for file in $( find ${root}src ${root}drafts ${root}later -type f -name "*.md"  -not -name "refs.incl.md" -not -name "SUMMARY.md" -not -name "*refs.md" )
do
  #echo ">> $file"
  # Ignore Rust code
  contents=$(cat "$file" ) #rg --multiline --invert-match '`.*?`' "$file")
  # Return the contents of the parens for [...](...)
  for in_parens in $( rg --no-line-number --no-filename --only-matching --replace='$2' '(\[[^\]]*\])\(([^)]*)\)' <<< "${contents}" )
  do
    # Skip [...](https://github.com/john-cd/rust_howto/issues/...)
    [[ "$in_parens" =~ ^https://github.com/john-cd/rust_howto/issues/[0-9]+$ ]] && continue
    # Get the label from the refdef files
    label=$(rg --no-line-number --no-filename -r '$1' '^(.+?): '"${in_parens}"'\s*$' "${root}/src/refs/")
    if [ -n "$label" ]; then
        #echo "${file} >> ${in_parens} --> ${label}"
        ## Replace the link in parens by the label. Using < as a separator, because it is not a valid URL character
        sed -i -E 's<\('"${in_parens}"'\)<'"${label}"'<g' "${file}"
    fi
    if [[ -z "$label" ]]; then
        #echo "${in_parens}"

        # Print crate names that are missing in the refdefs
        crate=$(sed -n -E -e 's<https://crates.io/crates/(.*)<\1<p' <<< "${in_parens}")
        if [ -n "$crate" ]; then
          just u templ b -a "$crate"
        fi

        crate=$(sed -n -E -e 's<https://lib.rs/crates/(.*)<\1<p' <<< "${in_parens}")
        if [ -n "$crate" ]; then
          just u templ b -a "$crate"
        fi

        ## https://github.com/ImplFerris/LearnRust
        #sed -n -E 's<(https://github.com/[^/]+/)(.+)<[\2~repo]: \1\2<p' <<< "${in_parens}"

        # Create refdefs for books:
        #sed -n -E 's<(https://en.wikipedia.org/wiki/Special:BookSources/)(.*)<[ISBN~\2]: \1\2<p' <<< "${in_parens}"

        ## For videos: https://www.youtube.com/@rustfoundation
        #sed -n -E 's<(https://www.youtube.com/)(@.*)<[youtube~\2]: \1\2<p' <<< "${in_parens}"

        # Playlists: https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa
        #sed -n -E 's<(https://www.youtube.com/playlist?list=)(.*)<[youtube~\2]: \1\2<p' <<< "${in_parens}"

        # https://medium.com/tag/rust
        #sed -n -E 's<(https://medium.com/)(.*)<[blog~medium~\2]: \1\2<p' <<< "${in_parens}"

        ## Meetups: https://www.meetup.com/Rust-Paris
        #sed -n -E 's<(https://www.meetup.com/)(.*)<[meetup~\2]: \1\2<p' <<< "${in_parens}"

        ## Books: https://www.amazon.com/Beginning-Rust-Get-Started-2021/dp/1484272072
        #sed -n -E 's<(https://www.amazon.com/)([^/]+)(/.*)<[book~\2]: \1\2\3<p' <<< "${in_parens}"

        # SO: https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait
        #sed -n -E 's<(https://stackoverflow.com/questions/)([^/]+)(/.*)<[blog~stackoverflow~\3]: \1\2\3<p' <<< "${in_parens}"

        # Podcasts: https://podcasts.apple.com/us/podcast/are-we-podcast-yet/id1484368019
        #sed -n -E 's<(https://podcasts.apple.com/us/podcast/)([^/]+)(/.*)<[podcast~\2]: \1\2\3<p' <<< "${in_parens}"

        # https://doc.rust-lang.org/reference/items/use-declarations.html
        #sed -n -E 's<(https://doc.rust-lang.org/reference/)(.*)<[book~rust-reference~\2]: \1\2<p' <<< "${in_parens}"

        # https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
        #sed -n -E 's<(https://doc.rust-lang.org/book/)(.*)<[book~rust~\2]: \1\2<p' <<< "${in_parens}"

        # https://poignardazur.github.io
        #sed -n -E 's<(https://)([^.]+)(.github.io)<[blog~\2]: \1\2\3<p' <<< "${in_parens}"

        # https://pypi.org/project/pip-tools
        #sed -n -E 's<(https://pypi.org/project/)(.*)<[pypi~\2]: \1\2<p' <<< "${in_parens}"

        # https://doc.rust-lang.org/stable/cargo/commands/cargo-fmt.html
        # https://doc.rust-lang.org/cargo/commands/cargo-test.html
        #sed -n -E 's<https://doc.rust-lang.org/(stable/)?cargo/commands/([^.]*)\.html<[book~cargo~\2]: &<p' <<< "${in_parens}"

        # https://www.reddit.com/r/rust/comments/1fyown4/rust_gpu_the_future_of_gpu_programming
        #sed -n -E 's<(https://www.reddit.com/r/rust/comments/)([^/]+/)(.*)<[reddit~\3]: &<p' <<< "${in_parens}"

        ## https://doc.rust-lang.org/std/cmp/index.html
        ## https://doc.rust-lang.org/std/result/enum.Result.html
        #sed -n -E -e 's<https://doc.rust-lang.org/std/([^/]+)$<[c~std::\1~docs]: &<p' -e 's<https://doc.rust-lang.org/std/([^/]+)/[^.]+\.([^.]+)\.html<[c~std::\1::\2~docs]: &<p' <<< "${in_parens}"

        # [keyword~dyn]: https://doc.rust-lang.org/std/keyword.dyn.html
        #sed -n -E 's<https://doc.rust-lang.org/std/keyword\.(.*).html<[keyword~\1]: &<p' <<< "${in_parens}"

        # https://doc.rust-lang.org/std/macro.todo.html
        #sed -n -E -e 's<https://doc.rust-lang.org/std/[^.]+\.([^.]+)\.html$<[c~std::\1~docs]: &<p' <<< "${in_parens}"

        # https://docs.rs/dyn-clone/latest/dyn_clone/trait.DynClone.html
        sed -n -E -e 's<https://docs.rs/[^/]+/latest/([^/]+)/[^.]+\.([^.]+).html<[c~\1::\2~docs]: &<p' <<< "${in_parens}"

        # General case
        #sed -n -E 's<https://(.+)<[\1~website]: &<p' <<< "${in_parens}"
    fi
  done
done

echo "DONE"
