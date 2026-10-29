// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Loads inline Fluent resources and formats greetings and plural messages.
//!
//! The example keeps English and French translations in FTL resources and
//! selects complete messages for each locale.

use std::error::Error;
use std::io;

use fluent_bundle::FluentArgs;
use fluent_bundle::FluentBundle;
use fluent_bundle::FluentResource;
use unic_langid::LanguageIdentifier;

/// The fallible result type used by this standalone example.
type ExampleResult<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

/// English messages with a greeting and singular/plural item counts.
const EN_US: &str = "greeting = Hello, { $name }!\nitems = { $count ->\n    [one] You have one item.\n   *[other] You have { $count } items.\n}\n";

/// French messages with a greeting and singular/plural item counts.
const FR: &str = "greeting = Bonjour, { $name } !\nitems = { $count ->\n    [one] Vous avez { $count } article.\n   *[other] Vous avez { $count } articles.\n}\n";

/// Builds a Fluent bundle for a locale from an inline FTL resource.
///
/// # Errors
/// Returns an error when the locale cannot be parsed, the resource is invalid,
/// or the resource cannot be added to the bundle.
fn new_bundle(
    locale: &str,
    source: &str,
) -> ExampleResult<FluentBundle<FluentResource>> {
    let locale = locale.parse::<LanguageIdentifier>()?;
    let resource =
        FluentResource::try_new(source.to_owned()).map_err(|(_, errors)| {
            io::Error::other(format!(
                "could not parse Fluent resource: {errors:?}"
            ))
        })?;
    let mut bundle = FluentBundle::new(vec![locale]);
    bundle.set_use_isolating(false);
    bundle.add_resource(resource).map_err(|errors| {
        io::Error::other(format!("could not add Fluent resource: {errors:?}"))
    })?;
    Ok(bundle)
}

/// Formats a message from a bundle and reports missing messages or arguments.
///
/// # Errors
/// Returns an error when the message has no value or Fluent reports a
/// formatting problem.
fn format_message(
    bundle: &FluentBundle<FluentResource>,
    message_id: &str,
    args: Option<&FluentArgs<'_>>,
) -> ExampleResult<String> {
    let message = bundle.get_message(message_id).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Fluent message `{message_id}` was not found"),
        )
    })?;
    let pattern = message.value().ok_or_else(|| {
        io::Error::other(format!("Fluent message `{message_id}` has no value"))
    })?;
    let mut errors = Vec::new();
    let formatted = bundle.format_pattern(pattern, args, &mut errors);
    if !errors.is_empty() {
        return Err(io::Error::other(format!(
            "could not format Fluent message `{message_id}`: {errors:?}"
        ))
        .into());
    }
    Ok(formatted.into_owned())
}

/// Prints English and French greetings and pluralized item counts.
fn main() -> ExampleResult<()> {
    let english = new_bundle("en-US", EN_US)?;
    let french = new_bundle("fr", FR)?;

    let mut greeting_args = FluentArgs::new();
    greeting_args.set("name", "Ada");
    println!(
        "{}",
        format_message(&english, "greeting", Some(&greeting_args))?
    );
    println!(
        "{}",
        format_message(&french, "greeting", Some(&greeting_args))?
    );

    let mut count_args = FluentArgs::new();
    count_args.set("count", 2);
    println!("{}", format_message(&english, "items", Some(&count_args))?);
    println!("{}", format_message(&french, "items", Some(&count_args))?);
    Ok(())
}
// ANCHOR_END: example

/// Runs the Fluent localization example.
pub fn run() -> ExampleResult<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies translated greetings and singular and plural forms in both
    /// locales.
    #[test]
    fn formats_english_and_french_messages() -> ExampleResult<()> {
        let english = new_bundle("en-US", EN_US)?;
        let french = new_bundle("fr", FR)?;

        let mut greeting_args = FluentArgs::new();
        greeting_args.set("name", "Ada");
        assert_eq!(
            format_message(&english, "greeting", Some(&greeting_args))?,
            "Hello, Ada!"
        );
        assert_eq!(
            format_message(&french, "greeting", Some(&greeting_args))?,
            "Bonjour, Ada !"
        );

        let mut singular_args = FluentArgs::new();
        singular_args.set("count", 1);
        assert_eq!(
            format_message(&english, "items", Some(&singular_args))?,
            "You have one item."
        );
        assert_eq!(
            format_message(&french, "items", Some(&singular_args))?,
            "Vous avez 1 article."
        );

        let mut plural_args = FluentArgs::new();
        plural_args.set("count", 2);
        assert_eq!(
            format_message(&english, "items", Some(&plural_args))?,
            "You have 2 items."
        );
        assert_eq!(
            format_message(&french, "items", Some(&plural_args))?,
            "Vous avez 2 articles."
        );
        run()
    }

    #[test]
    fn test_main_entry_point() -> ExampleResult<()> {
        main()
    }

    #[test]
    fn unknown_message_is_a_not_found_error() -> ExampleResult<()> {
        let english = new_bundle("en-US", EN_US)?;
        let error = format_message(&english, "missing", None)
            .expect_err("the bundle has no such message");
        let io_error = error
            .downcast_ref::<io::Error>()
            .expect("the error comes from the example");
        assert_eq!(io_error.kind(), io::ErrorKind::NotFound);
        Ok(())
    }

    #[test]
    fn missing_argument_is_reported() -> ExampleResult<()> {
        let english = new_bundle("en-US", EN_US)?;
        assert!(format_message(&english, "greeting", None).is_err());
        Ok(())
    }

    #[test]
    fn invalid_resource_and_locale_are_rejected() {
        assert!(new_bundle("en-US", "greeting = { $name").is_err());
        assert!(new_bundle("not a locale!", EN_US).is_err());
    }
}
