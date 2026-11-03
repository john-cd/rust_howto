# Local security backport

This is the crates.io `zenoh-transport` 1.10.2 source. Its only source change
is the `lz4_flex` dependency bump from 0.10.0 to 0.11.6, matching the upstream
Dependabot change `eclipse-zenoh/zenoh@1ad912b0936a5ed6cf556b48feae363c194c5a75`.
Keep the patch until a Zenoh release includes that fix.
