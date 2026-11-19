//! Sorts the same words using German and Swedish collation.

// ANCHOR: example
use icu::locale::locale;
use icu_collator::Collator;

/// Checks the different positions of ä under German and Swedish collation.
///
/// # Errors
///
/// Returns an error if a collator cannot load its locale data.
fn main() -> anyhow::Result<()> {
    let german = Collator::try_new(locale!("de").into(), Default::default())?;
    let swedish = Collator::try_new(locale!("sv").into(), Default::default())?;

    let mut words = ["z", "ä", "b"];
    words.sort_by(|left, right| german.compare(left, right));
    assert_eq!(words, ["ä", "b", "z"]);

    words.sort_by(|left, right| swedish.compare(left, right));
    assert_eq!(words, ["b", "z", "ä"]);
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Confirms that German and Swedish put ä on opposite sides of z.
    #[test]
    fn sorts_with_locale_rules() -> anyhow::Result<()> {
        super::main()
    }
}
