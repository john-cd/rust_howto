use std::borrow::Cow;
use std::collections::BTreeMap;

use tracing::info;

use crate::dependencies::Dependency;
use crate::link::Link;

/// Create reference definitions from dependencies
///
/// dependencies: sorted map of dependencies
pub(crate) fn generate_refdefs_from(
    dependencies: BTreeMap<Cow<'_, str>, Dependency>,
) -> Vec<Link<'static>> {
    let mut buf = Vec::new();
    for (_, dep) in dependencies {
        info!("{:?}", dep);
        buf.push(generate_refdefs_for_one_library(
            &dep.library_name,
            dep.package_repo_url.as_deref(),
        ));
    }
    buf.into_iter().flatten().collect::<Vec<_>>()
}

// TODO
/// Create, for a given crate, multiple reference definitions for
/// common websites such as docs.rs, crates.io, github,
/// and th associated badge URLs
fn generate_refdefs_for_one_library(
    library_name: &str,
    package_repo_url: Option<&str>,
) -> Vec<Link<'static>> {
    //let mut buf = Vec::new();

    // // [arrow-rs]: https://docs.rs/arrow/
    // let docs_rs_url = format!("https://docs.rs/{}", library_name);

    // // [config-crate]: https://crates.io/crates/config/
    // let crates_io_lbl = format!("{}-crate", library_name);
    // let crates_io_url = format!("https://crates.io/crates/{}", library_name);

    // // [arrow-rs-github]: https://github.com/apache/arrow-rs/
    // let github_lbl = format!("{}-github", library_name);

    // info!("{}", badge_image_url.to_string());
    // let link = crate::link::LinkBuilder::default()
    //     .set_label(library_name)
    //     //.set_url()
    //     .set_image_url(badge_image_url.to_string())
    //     .build();

    //buf

    Vec::new()
}
