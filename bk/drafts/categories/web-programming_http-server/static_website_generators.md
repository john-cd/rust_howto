<!-- TODO Reviewed ??-??-2026 -->

# Static Website Generators

{{#include static_website_generators.incl.md}}

Static site generators turn source content into files that can be served by a static web server or content delivery network. They work well when pages can be generated ahead of time and do not need a server-side database for each request. Choose a tool based on the kind of site, the amount of interactivity, and the build and deployment environment.

| Tool                                     | Good fit                                                                 | Considerations                                                                                                                      |
| ---------------------------------------- | ------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------- |
| [Zola][zola~website]↗{{hi:zola}}         | Blogs, portfolios, and content-focused websites                          | A single generator provides templates, Markdown processing, assets, and a static build.                                             |
| [`mdBook`][c~mdbook~docs]↗{{hi:mdBook}}  | Technical books and structured documentation                             | Markdown chapters, navigation, search, and documentation-oriented workflows; it is not a general-purpose content management system. |
| [Dioxus][c~dioxus~website]↗{{hi:Dioxus}} | Rust applications that need interactive pages or static route generation | Shares UI components across rendering targets; more application structure is needed than for a simple content site.                 |

## Build a Content Site with Zola {#zola}

[![zola][zola~website~badge]][zola~website] [![zola~repo][c~zola~repo~badge]][zola~repo]{{hi:zola}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

[Zola][zola~website]↗{{hi:zola}} generates a complete static site from templates, Markdown content, and assets. Install the release for your platform, initialize a project with `zola init`, then add pages under the content directory. Configure the site's base URL, theme, and build options in its configuration file.

Use `zola serve` to preview changes locally and `zola build` to produce the deployable site in the `public` directory. Publish that directory to a static host. The generated site has no request-time database; features that need dynamic data require a separate service or client-side code.

For a technical book with a chapter hierarchy and built-in search, use [`mdBook`][c~mdbook~docs]↗{{hi:mdBook}}. Start with the [mdBook guide to creating a book][book~mdbook~creating]↗ and its [installation instructions][book~mdbook~installation]↗. For a Rust UI that needs interactive behavior or static route generation, see [Dioxus static site generation][dioxus~ssg]↗.

## Related Topics {#related-topics .skip}

- [Zola documentation][zola~website]↗.
- [mdBook documentation][c~mdbook~docs]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
