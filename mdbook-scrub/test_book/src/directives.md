# Preprocessor Directives

The {{ ... }} blocks below should be replaced by (a) Markdown link(s).

## Page Link

<https://en.wikipedia.org/wiki/Help:Link#Wikilinks_(internal_links)>

[[file]]         -> [file header](file)
[[file | title]] -> [title](file)

[[file]]         -> [file][p-file]
[[file | title]] -> [title][p-file]

[scraping][p-scraping]

- [[scraping]].

- [[scraping | Scraping]].

- [[science_geo]].


FIXME finish below

## Just the Crate Link

[`mdbook-linkcheck`][c-mdbook_linkcheck-github]{{hi:mdbook-linkcheck}}⮳.

{{c: xyz}}

{{c: xyz }}

{{c: x_y-z }}


## Crate Badge

[![num][c-num-badge]][c-num]{{hi:num}}⮳

{{c_b: xyz}}

{{c_b: xyz }}

{{c_b: x_y-z}}


## Just the Category Link

[testing][cat-development-tools::testing]⮳{{hi:development-tools::testing}}

[Accessibility][cat-accessibility]⮳{{hi:accessibility}}

[Testing][cat-development-tools::testing]⮳{{hi:development-tools::testing}}

{{cat: xyz}}

{{cat: xyz }}

{{cat: x-y_z::a-b_c }}

{{cat: xyz | XYZ }}


## Category Badge

[![cat-mathematics][cat-mathematics-badge]][cat-mathematics]

[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

{{cat_b: mathematics}}

{{cat_b: mathematics }}

{{cat_b: mathematics science }}

{{cat_b: no-std }}

{{cat_b: no_std }}


## Crate Block

{{crate: crt }}

{{crate: crt}}

{{crate: x_y-z }}

{{crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}
