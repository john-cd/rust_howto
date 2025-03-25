# Wikilinks

The `rust_howto` book uses wikilinks to insert links to other pages.

See <https://en.wikipedia.org/wiki/Help:Link#Wikilinks_(internal_links)>.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding
configuration toggle is set (which is the default).

The following wikilinks should be replaced by their `title`, if present:

~[[]]~

~[[file]]~

~[[ file ]]~

~[[ file |]]~

~[[ file |  ]]~

~[[ file | title1 ]]~

~[[file|title2]]~

~[[ file|title3]]~

~[[file |title4]]~

~[[file| title5]]~

~[[file|title6 ]]~

~[[|title7]]~

~[[| title8]]~

~[[|title9 ]]~

~[[| title10 ]]~
