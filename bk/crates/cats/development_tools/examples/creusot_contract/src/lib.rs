// TODO Reviewed ??-??-2026.
use creusot_std::prelude::*;

// ANCHOR: example
#[requires(x@ < i64::MAX@)]
#[ensures(result@ == x@ + 1)]
pub fn add_one(x: i64) -> i64 {
    x + 1
}

#[cfg(feature = "deliberately-fails")]
#[requires(x@ < i64::MAX@)]
#[ensures(result@ == x@ + 1)]
pub fn add_one_wrong(x: i64) -> i64 {
    x
}
// ANCHOR_END: example
