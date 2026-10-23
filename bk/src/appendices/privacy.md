<!-- TODO Reviewed ??-??-2026 -->
# Privacy

The published book loads a Google Analytics 4 tag to measure page views and
site usage. Google Analytics' default collection includes user and session
statistics, approximate location, browser and device information, and page
URLs and titles. The tag uses a first-party `_ga` cookie to distinguish
visitors. Google says that Google Analytics 4 uses IP addresses
at collection time to estimate location and discards them before measurement
data is logged.

The analytics tag loads only when this book is served over HTTPS from
`john-cd.com` or `www.john-cd.com` at `/rust_howto/`. Local builds and preview
hosts do not load the Google Analytics script.

See Google's [Analytics data collection][google-analytics~data-collection]↗,
[regional data collection][google-analytics~regional-data]↗, and [privacy
information][google-analytics~privacy]↗.

{{#include ../refs/link-refs.md}}
