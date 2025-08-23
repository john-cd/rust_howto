// [test new templating](https://github.com/john-cd/rust_howto/issues/1431)

// /// Replaces the following directives by the corresponding markdown, in a given file:
// ///
// /// {{!crate xyz}}
// /// {{!docs xyz}}
// /// {{!github xyz}}
// /// {{!lib.rs xyz}}
// /// {{!crates.io xyz}}
// /// {{!web xyz}}
// ///
// pub fn process_crate_badge_directives_in_file(filepath: &std::path::Path) -> anyhow::Result<()> {
//     core_lib::process_file(filepath, has_crate_badge, process_crate_badge_directives)?;
//     Ok(())
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_replace_crate() {
//         let text = "{{!crate clap regex}}";
//         let expected = "[![clap][p~clap~crate~badge]][p~clap~crate]{{hi:clap}}[![regex][p~regex~crate~badge]][p~regex~crate]{{hi:regex}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_docs() {
//         let text = "{{!docs clap regex}}";
//         let expected = "[![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}}[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_github() {
//         let text = "{{!github clap regex}}";
//         let expected = "[![clap][c~clap~repo~badge]][c~clap~repo]{{hi:clap}}[![regex][c~regex~repo~badge]][c~regex~repo]{{hi:regex}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_lib_rs() {
//         let text = "{{!lib.rs clap regex}}";
//         let expected = "[![clap][c~clap~lib.rs~badge]][c~clap~lib.rs]{{hi:clap}}[![regex][c~regex~lib.rs~badge]][c~regex~lib.rs]{{hi:regex}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_crates_io() {
//         let text = "{{!crates.io clap regex}}";
//         let expected = "[![clap][c~clap~crates.io~badge]][c~clap~crates.io]{{hi:clap}}[![regex][c~regex~crates.io~badge]][c~regex~crates.io]{{hi:regex}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_web() {
//         let text = "{{!web clap regex}}";
//         let expected = "[![clap][c~clap~website~badge]][c~clap~website]{{hi:clap}}[![regex][c~regex~website~badge]][c~regex~website]{{hi:regex}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_with_extra_spaces() {
//         let text = "{{   !   docs   clap   regex   }}";
//         let expected = "[![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}}[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_multiple_types() {
//         let text = "{{!docs : regex}} {{!github: walkdir}}";
//         let expected = "[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![walkdir][c~walkdir~repo~badge]][c~walkdir~repo]{{hi:walkdir}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_no_match() {
//         let text = "This is a test string with no matches.";
//         let expected = "This is a test string with no matches.";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_empty_input() {
//         let text = "";
//         let expected = "";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_no_crate_name() {
//         let text = "{{!crate }}";
//         let expected = "{{!crate }}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_single_crate() {
//         let text = "{{!crate clap}}";
//         let expected = "[![clap][p~clap~crate~badge]][p~clap~crate]{{hi:clap}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }
//     #[test]
//     fn test_replace_single_docs() {
//         let text = "{{!docs clap}}";
//         let expected = "[![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }
//     #[test]
//     fn test_replace_single_github() {
//         let text = "{{!github clap}}";
//         let expected = "[![clap][c~clap~repo~badge]][c~clap~repo]{{hi:clap}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }
//     #[test]
//     fn test_replace_single_lib_rs() {
//         let text = "{{!lib.rs clap}}";
//         let expected = "[![clap][c~clap~lib.rs~badge]][c~clap~lib.rs]{{hi:clap}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }
//     #[test]
//     fn test_replace_single_crates_io() {
//         let text = "{{!crates.io clap}}";
//         let expected = "[![clap][c~clap~crates.io~badge]][c~clap~crates.io]{{hi:clap}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }
//     #[test]
//     fn test_replace_single_web() {
//         let text = "{{!web clap}}";
//         let expected = "[![clap][c~clap~website~badge]][c~clap~website]{{hi:clap}}";
//         assert_eq!(process_crate_badge_directives(text), expected);
//     }

//     #[test]
//     fn test_replace_in_file_no_match() -> anyhow::Result<()> {
//         use core_lib::TestCase;
//         use core_lib::test_with;
//         use core_lib::walk_directory_and_process_files;

//         let test_cases = vec![
//             // No match.
//             TestCase {
//                 file_name_and_ext: "test1.md",
//                 original_contents: Some("No changes."),
//                 expected_final_contents: Some("No changes."),
//             },
//             // Single match.
//             TestCase {
//                 file_name_and_ext: "test2.md",
//                 original_contents: Some("This is a test file: {{!docs clap}}."),
//                 expected_final_contents: Some(
//                     "This is a test file: [![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}}.",
//                 ),
//             },
//             // Empty file.
//             TestCase {
//                 file_name_and_ext: "test3.md",
//                 original_contents: None,
//                 expected_final_contents: None,
//             },
//         ];

//         let scope = core_lib::Scope::default();

//         test_with(&test_cases, |temp_dir_path| {
//             walk_directory_and_process_files(
//                 temp_dir_path,
//                 &scope,
//                 process_crate_badge_directives_in_file,
//             )?;
//             Ok(())
//         })?;

//         Ok(())
//     }
// }
