// use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use tool_lib::create_crate_badges_or_refdefs;

/// Creates a badge for a crate, including its name, keywords, categories, description, and reference definitions.
///
/// # Arguments
///
/// * `name` - The name of the crate.
pub(super) fn create_crate_badge_with_categories(name: &str) -> Result<(String, Vec<String>)> {
    let mut s = String::new();
    let info =
        tool_lib::get_info_for_crate(name).with_context(|| format!("Unknown crate: {name}"))?;

    let badges =
        create_crate_badges_or_refdefs(&info.crate_data, tool_lib::GenerationMode::CrateBadges)?;
    s.push_str(&badges);

    // Add index entries for keywords
    let keywords: Vec<_> = info
        .keywords
        .into_iter()
        .map(|k| capitalize_if_not(k.keyword, name))
        .collect();
    let kws = keywords.iter().map(String::as_str).collect();
    let markdown = tool_lib::create_index_anchors(kws)?;
    s.push_str(&markdown);

    // Print category badges
    for cat in info.categories {
        let markdown = tool_lib::create_category_badge(&cat.category, &cat.slug)?;
        s.push_str(&markdown);
    }
    s.push_str("\n\n");

    let desc = create_crate_badges_or_refdefs(
        &info.crate_data,
        tool_lib::GenerationMode::CrateDescription,
    )?;
    if !desc.is_empty() {
        s.push_str(&(format!("{desc}\n\n")));
    }

    let refdefs =
        create_crate_badges_or_refdefs(&info.crate_data, tool_lib::GenerationMode::CrateRefdefs)?;
    s.push_str(&refdefs);

    let vector_of_lines: Vec<String> = refdefs.split('\n').map(|str| str.to_string()).collect();

    Ok((s, vector_of_lines))
}

/// Capitalizes the first letter of a string if it is not equal to another string.
///
/// # Arguments
///
/// * `s` - The string to capitalize.
/// * `should_not_be` - The string that `s` should not be equal to.
fn capitalize_if_not(s: String, should_not_be: &str) -> String {
    if s != should_not_be {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    } else {
        s
    }
}
