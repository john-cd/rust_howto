# Date and Time

Manage the complexity of dealing with the fourth dimension.

## Duration and Calculation

{{#include duration.incl.md}}

## Parsing and Displaying

{{#include parse.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO cover https://crates.io/crates/time - see blessed.rs

[![time][c-time-badge]][c-time]
[![time-crates.io][c-time-crates.io-badge]][c-time-crates.io]
[![time-github][c-time-github-badge]][c-time-github]
[![time-lib.rs][c-time-lib.rs-badge]][c-time-lib.rs]

Unfortunately there is no clear answer as to which is best between time and chrono.
Evaluate for yourself between these two, but be resassured that both are trusted and well-maintained.

time
A smaller, simpler library. Preferrable if covers your needs, but it's quite limited in what it provides.

chrono
The most comprehensive and full-featured datetime library, but more complex because of it.
</div>
