// use super::*;

// // * Category links:
// //
// // {{cat xyz}}
// //
// // Variants:
// //
// // {{cat xyz  }}
// // {{  cat xyz}}
// // {{cat: xyz}}
// // {{cat  :  xyz}}
// // {{cat x-y_z::a-b_c }}

// // TODO
// #[test]
// fn test_category_link_regexes() {
//     let text = "{{cat xyz}}";
//     let expected = "xyz";
//     let r = category_link_regexes();
//     let res = r[0]
//         .re
//         .replace_all(text, r[0].replacement.as_ref().unwrap());
//     assert_eq!(res, expected);
// }

// // * Category badges:
// //
// // {{!cat no_std}}
// // {{!cat mathematics }}
// // {{ !cat mathematics}}
// // {{!cat: mathematics}}
// // {{!cat : mathematics}}

// // TODO
// #[test]
// fn test_category_badge_regexes() {
//     let text = "{{cat xyz}}";
//     let expected = "xyz";
//     let r = category_badge_regexes();
//     let res = r[0]
//         .re
//         .replace_all(text, r[0].replacement.as_ref().unwrap());
//     assert_eq!(res, expected);
// }
