// // List build info, incl. dependencies
// //
// // See
// // https://crates.io/crates/build-info
// // https://stackoverflow.com/questions/41618456/get-list-of-active-dependencies-and-their-versions-during-cargo-build

// // In build.rs add:
// // use build_info_build::DependencyDepth;
// // // Collect build info to later generate reference definitions
// for all dependencies... // build_info_build::build_script().
// collect_dependencies(DependencyDepth::Depth(1));

// use std::collections::BTreeSet;

// // Create the buil_info function: println!("{:#?}", build_info());
// build_info::build_info!(fn build_info);

// // Collect all of the dependencies of this workspace into a
// singleset. fn get_dependencies() -> BTreeSet<(&'static str,
// &'static build_info::semver::Version)> {     // called recursively
// on each of the dependencies in the tree     fn visit(
//         info: &'static build_info::CrateInfo,
//         set: &mut BTreeSet<(&'static str, &'static
//         build_info::semver::Version)>,
//     ) {
//         set.insert((&info.name, &info.version));
//         for dep in &info.dependencies {
//             visit(dep, set);
//         }
//     }
//     let mut set = BTreeSet::new();
//     let root_info = &build_info().crate_info;
//     visit(root_info, &mut set);
//     set
// }

// fn list_deps() {
//     println!("Dependencies:");
//     for (name, version) in get_dependencies() {
//         println!("{} {}", name, version);
//     }
// }

// fn main() {
//     list_deps();
// }

fn main() {}
// [WIP review / convert into example](https://github.com/john-cd/rust_howto/issues/1060)?
