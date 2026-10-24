<!-- TODO Reviewed ??-??-2026 -->

# Extracting Links

{{#include scraping.incl.md}}

These examples use [`scraper`][c~scraper~docs]↗ for CSS-selector-based HTML parsing instead of the earlier `select` API.

## Extract all Links from the HTML of a Webpage {#extract-all-links-from-a-webpage-html}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![scraper][c~scraper~docs~badge]][c~scraper~docs]{{hi:scraper}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Uses [`reqwest::get`][c~reqwest::get~docs]↗{{hi:reqwest::get}} to fetch the page and [`scraper::Html::parse_document`][c~scraper::Html::parse_document~docs]↗{{hi:scraper::Html::parse_document}} to parse its HTML. A CSS [`scraper::Selector`][c~scraper::Selector~docs]↗{{hi:scraper::Selector}} finds anchors with an `href` attribute.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/scraping/extract_links.rs:example}}
```

## Check a Webpage for Broken Links {#check-a-webpage-for-broken-links}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![scraper][c~scraper~docs~badge]][c~scraper~docs]{{hi:scraper}} [![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

The example resolves the first HTML `<base href>` against the page URL. If there is no base element, relative links resolve against the full document URL. It follows the URL rules in [[url | Uniform Resource Locator]] and checks only HTTP(S) links; `mailto:` and other schemes are skipped.

The checker makes requests sequentially with a finite per-request timeout, so a page cannot create an unbounded number of concurrent tasks. It reports `404 Not Found` and `410 Gone` as broken links; other unexpected HTTP statuses and transport errors are returned as errors.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/scraping/broken.rs:example}}
```

## Extract all Unique Links from a MediaWiki Markup {#extract-all-unique-links-from-a-mediawiki-markup}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]

Pull a MediaWiki source page using [`reqwest::get`][c~reqwest::get~docs]↗{{hi:reqwest::get}} and find internal and external links with [`regex::Regex::captures_iter`][c~regex::Regex::captures_iter~docs]↗{{hi:regex::Regex::captures_iter}}. This regular expression demonstrates a small subset of wikitext link syntax; it is not a general HTML link extractor. Using [`std::borrow::Cow`][c~std::borrow::Cow~docs]↗{{hi:std::borrow::Cow}} avoids allocating strings for every captured link.

MediaWiki link syntax is described [here][mediawiki~link-syntax]↗.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/scraping/unique.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[url | Uniform Resource Locator]].
- [[requests | Make HTTP Requests]].

HTML parsing does not execute page JavaScript. For pages that need browser rendering or interaction, consider [headless_chrome](https://github.com/rust-headless-chrome/rust-headless-chrome) (Chrome DevTools Protocol), [chromiumoxide](https://github.com/mattsse/chromiumoxide) (Chrome DevTools Protocol), or [fantoccini](https://github.com/jonhoo/fantoccini) (WebDriver).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
This review replaces the older `select` API with `scraper`, fixes base URL resolution and HTTP status handling, and adds deterministic local HTML tests. Browser automation alternatives are visible for sites that depend on JavaScript.
</div>
