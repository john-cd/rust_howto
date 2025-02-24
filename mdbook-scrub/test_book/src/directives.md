# Preprocessor directives

The {{ ... }} blocks below should be replaced by (a) Markdown link(s).

## Page link

<https://en.wikipedia.org/wiki/Help:Link#Wikilinks_(internal_links)>

[[file]]         -> [file header](file)
[[file | title]] -> [title](file)

[[file]]         -> [file][p-file]
[[file | title]] -> [title][p-file]

[scraping][p-scraping]

[[scraping]]

[[scraping | Scraping]]

[[science_geo]]


## Just the crate link

[`mdbook-linkcheck`][c-mdbook_linkcheck-github]{{hi:mdbook-linkcheck}}⮳

{{c: xyz}}

{{c: xyz }}

{{c: x_y-z }}


## Crate badge

[![num][c-num-badge]][c-num]{{hi:num}}⮳

{{c_b: xyz}}

{{c_b: xyz }}

{{c_b: x_y-z}}


## Just the category link

[testing][cat-development-tools::testing]⮳{{hi:development-tools::testing}}

[Accessibility][cat-accessibility]⮳{{hi:accessibility}}

[Testing][cat-development-tools::testing]⮳{{hi:development-tools::testing}}

{{cat: xyz}}

{{cat: xyz }}

{{cat: x-y_z::a-b_c }}

{{cat: xyz | XYZ }}


## Category badge

[![cat-mathematics][cat-mathematics-badge]][cat-mathematics]

[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

{{cat_b: mathematics}}

{{cat_b: mathematics }}

{{cat_b: mathematics science }}

{{cat_b: no-std }}

{{cat_b: no_std }}


## Crate block

{{crate: crt }}

{{crate: crt}}

{{crate: x_y-z }}

{{crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}
