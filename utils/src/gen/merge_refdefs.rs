// /// Merge existing refdefs and new ones and write the result to
// file use crate::link::Link;

// // Append, sort and dedupe reference definitions
// pub(crate) fn merge_links<'a, 'b>(
//     existing_links: Vec<Link<'a>>,
//     mut new_links: Vec<Link<'b>>,
// ) -> Vec<Link<'a>>
// where
//     'b: 'a
// {
//     let mut buf = existing_links.clone();
//     buf.append(&mut new_links);

//     // `Link` has a custom Ord / Eq implementation, thus we can
// sort them.     buf.sort();
//     buf.dedup();
//     buf
// }
