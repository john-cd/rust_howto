<!-- TODO Reviewed ??-??-2026 -->

# Kindle Distribution Evaluation

Reviewed 2026-10-22. This is a distribution assessment, not a decision to sell or publish the book.

## Options

| Route | Fit | Trade-off |
| --- | --- | --- |
| Keep the existing website as the primary edition | Best for a frequently updated, openly readable technical book with working links and code snippets. | Readers need a browser connection; the site remains the canonical experience. |
| Publish a Kindle eBook through Kindle Direct Publishing (KDP) | KDP accepts EPUB and Kindle Package Format (KPF). KDP recommends previewing the converted book before publication. | Requires a separate eBook layout review, metadata and cover work, and an ongoing process for updates. Code-heavy pages, tables, and web links need device testing. |
| Offer an EPUB or PDF directly | A direct download can complement the website and can be distributed through the author's own site or a digital storefront such as Gumroad. | Adds a second release artifact to rebuild and keep current; storefront fees and checkout experience need a separate comparison. |

## Recommendation

Do not start a Kindle release until the existing Pandoc export plan produces a validated, reflowable EPUB or PDF from `bk/src`. First test a small set of chapters with code blocks, tables, links, and included content. If that output is readable on Kindle, prepare an EPUB and use Amazon's Kindle Previewer before uploading it to KDP. Kindle Create is an optional desktop preparation tool; it is not required when the exported EPUB already meets KDP's guidelines.

Keep any future KDP release outside KDP Select unless the author explicitly chooses digital exclusivity. KDP Select enrollment requires the Kindle eBook to remain exclusive to the Kindle Store during each 90-day enrollment period, which conflicts with simultaneously offering that same eBook through a direct store. A non-Select KDP listing can coexist with direct distribution, subject to the author's rights and chosen license.

## Source Checks

- KDP accepts EPUB and KPF eBook manuscripts and recommends previewing the uploaded book: [Supported eBook formats](https://kdp.amazon.com/en_US/help/topic/G200634390/) and [Kindle eBook manuscript formatting](https://kdp.amazon.com/en_US/help/topic/G200645680).
- Kindle Create exports KPF and EPUB, with KPF recommended for its full Kindle-specific formatting support: [Getting started with Kindle Create](https://kdp.amazon.com/en_US/help/topic/GUGQ4WDZ92F733GC).
- KDP Select is optional and imposes a 90-day Kindle Store exclusivity period on the enrolled eBook: [KDP Select enrollment](https://kdp.amazon.com/en_US/help/topic/GD9PMU58BV24QFZ7).
- Gumroad's current fee schedule distinguishes direct sales from marketplace discovery sales and may change; check its [current fees](https://gumroad.com/help/article/66-gumroads-fees) before choosing a storefront.

## Remaining Decision

The author should decide whether a Kindle edition is worth maintaining after a representative export has been previewed. Until then, keep the website as the published edition and treat Kindle and direct downloads as unapproved distribution options.
