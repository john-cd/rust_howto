# Web Programming

## Scraping Web Pages

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Extract all links from a webpage HTML][ex-extract-links-webpage] | [![reqwest-badge]][reqwest] [![select-badge]][select] | [![cat-net-badge]][cat-net] |
| [Check webpage for broken links][ex-check-broken-links] | [![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract all unique links from a MediaWiki markup][ex-extract-mediawiki-links] | [![reqwest-badge]][reqwest] [![regex-badge]][regex] | [![cat-net-badge]][cat-net] |

## Uniform Resource Locations (URL)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse a URL from a string to a `Url` type][ex-url-parse] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create a base URL by removing path segments][ex-url-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create new URLs from a base URL][ex-url-new-from-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract the URL origin (scheme / host / port)][ex-url-origin] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Remove fragment identifiers and query pairs from a URL][ex-url-rm-frag] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |

## Media Types (MIME)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Get MIME type from string][ex-mime-from-string] | [![mime-badge]][mime] | [![cat-encoding-badge]][cat-encoding] |
| [Get MIME type from filename][ex-mime-from-filename] | [![mime-badge]][mime] | [![cat-encoding-badge]][cat-encoding] |
| [Parse the MIME type of a HTTP response][ex-http-response-mime-type] | [![mime-badge]][mime] [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |

## Clients

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Make a HTTP GET request][ex-url-basic] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Query the GitHub API][ex-rest-get] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [Check if an API resource exists][ex-rest-head] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Create and delete Gist with GitHub API][ex-rest-post] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [Consume a paginated RESTful API][ex-paginated-api] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [Download a file to a temporary directory][ex-url-download] | [![reqwest-badge]][reqwest] [![tempdir-badge]][tempdir] | [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem] |
| [Make a partial download with HTTP range headers][ex-progress-with-range] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [POST a file to paste-rs][ex-file-post] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |

## Web Authentication

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Basic Authentication][ex-basic-authentication] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |

[ex-extract-links-webpage]: web/scraping.md#extract-all-links-from-a-webpage-html
[ex-check-broken-links]: web/scraping.md#check-a-webpage-for-broken-links
[ex-extract-mediawiki-links]: web/scraping.md#extract-all-unique-links-from-a-mediawiki-markup
[ex-url-parse]: web/url.md#parse-a-url-from-a-string-to-a-url-type
[ex-url-base]: web/url.md#create-a-base-url-by-removing-path-segments
[ex-url-new-from-base]: web/url.md#create-new-urls-from-a-base-url
[ex-url-origin]: web/url.md#extract-the-url-origin-scheme--host--port
[ex-url-rm-frag]: web/url.md#remove-fragment-identifiers-and-query-pairs-from-a-url
[ex-mime-from-string]: web/mime.md#get-mime-type-from-string
[ex-mime-from-filename]: web/mime.md#get-mime-type-from-filename
[ex-http-response-mime-type]: web/mime.md#parse-the-mime-type-of-a-http-response

[ex-url-basic]: web/clients/requests.md#make-a-http-get-request
[ex-rest-custom-params]: web/clients/requests.md#set-custom-headers-and-url-parameters-for-a-rest-request
[ex-rest-get]: web/clients/apis.md#query-the-github-api
[ex-rest-head]: web/clients/apis.md#check-if-an-api-resource-exists
[ex-rest-post]: web/clients/apis.md#create-and-delete-gist-with-github-api
[ex-paginated-api]: web/clients/apis.md#consume-a-paginated-restful-api
[ex-handle-rate-limited-api]: web/clients/apis.md#handle-a-rate-limited-api
[ex-url-download]: web/clients/download.md#download-a-file-to-a-temporary-directory
[ex-progress-with-range]: web/clients/download.md#make-a-partial-download-with-http-range-headers
[ex-file-post]: web/clients/download.md#post-a-file-to-paste-rs
[ex-basic-authentication]: web/clients/authentication.md#basic-authentication
{{#include refs/link-refs.md}}
