// #![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example shows how to use the `crates.io` API to retrieve the reverse
// //! dependencies of a crate. The API is paginated, so we need to make
// multiple //! requests to get all the dependencies.

// // FIXME
// // version_id then join version_id and Version.id to retrieve crate
// // name.
// // Consider a simpler API for example purposes.

// use anyhow::Result;
// use serde::Deserialize;

// // Structs used to deserialize the JSON produced by the API:

// #[derive(Deserialize)]
// struct ApiResponse {
//     dependencies: Vec<Dependency>,
//     // versions: Vec<Version>, // not used in this example
//     meta: Meta,
//     versions: Vec<Version>,
// }

// #[derive(Deserialize)]
// struct Dependency {
//     version_id: String,
//     crate_id: String,
// }

// #[derive(Deserialize)]
// struct Meta {
//     total: u32,
// }

// #[derive(Deserialize)]
// struct Version {
//     id: String,
//     // The API returns `crate` but we want to call it `crate_id`.
//     #[serde(alias = "crate")]
//     crate_id: String,
// }

// /// `ReverseDependencies` is a struct that retrieves the reverse dependencies
// of a crate from the /// `crates.io` API. It implements the `Iterator` trait
// to allow iterating over the dependencies. /// It handles pagination by making
// multiple requests to the API if necessary. ///
// struct ReverseDependencies {
//     crate_id: String,
//     dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
//     client: reqwest::blocking::Client,
//     page: u32,
//     per_page: u32,
//     total: u32,
// }

// impl ReverseDependencies {
//     /// Creates a new `ReverseDependencies` instance for the given crate ID.
//     ///
//     /// This function initializes the struct with default values and sets up
//     /// the HTTP client.
//     ///
//     /// # Arguments
//     ///
//     /// * `crate_id` - The name of the crate for which to retrieve reverse
// dependencies.     fn of(crate_id: &str) -> Result<Self> {
//         let client = reqwest::blocking::Client::builder()
//             .user_agent("Rust-test")
//             .build()?;
//         Ok(ReverseDependencies {
//             crate_id: crate_id.to_owned(),
//             dependencies: vec![].into_iter(),
//             client,
//             page: 0,
//             per_page: 100,
//             total: 0,
//         })
//     }

//     /// Attempts to retrieve the next dependency from the API.
//     ///
//     /// This function handles pagination by making multiple requests to the
// API     /// if necessary. It returns `Ok(Some(dep))` if a dependency is
// found,     /// `Ok(None)` if there are no more dependencies, and `Err(err)`
// if an error     /// occurs.
//     ///
//     /// This function is called internally by the `Iterator` implementation.
//     fn try_next(&mut self) -> Result<Option<Dependency>> {
//         if let Some(dep) = self.dependencies.next() {
//             return Ok(Some(dep));
//         }

//         if self.page > 0 && self.page * self.per_page >= self.total {
//             return Ok(None);
//         }

//         self.page += 1;
//         let url = format!(
//             "https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
//             self.crate_id, self.page, self.per_page
//         );
//         // println!("Calling {}", url);
//         let resp = self.client.get(url).send()?;
//         let json = resp.json::<ApiResponse>()?;

//         self.dependencies = json.dependencies.into_iter();
//         self.total = json.meta.total;
//         Ok(self.dependencies.next())
//     }
// }

// /// Implements the `Iterator` trait for `ReverseDependencies`.
// ///
// /// This allows iterating over the reverse dependencies of a crate. Each item
// /// returned by the iterator is a `Result<Dependency>`, which can be either
// /// `Ok(Dependency)` if a dependency is found, or `Err(err)` if an error
// /// occurred while retrieving the dependency. The iterator returns `None`
// when /// there are no more dependencies to retrieve.
// impl Iterator for ReverseDependencies {
//     type Item = Result<Dependency>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.try_next() {
//             Ok(Some(dep)) => Some(Ok(dep)),
//             Ok(None) => None,
//             Err(err) => Some(Err(err)),
//         }
//     }
// }

// fn main() -> Result<()> {
//     for dep in ReverseDependencies::of("bit-vec")? {
//         println!("Reverse dependency: {}", dep?.version_id);
//     }
//     Ok(())
// }

// #[test]
// fn require_network() -> anyhow::Result<()> {
//     let res = main();
//     println!("{:?}", res);
//     res?;
//     Ok(())
// }
// // [fix - the API no longer returns a crate_id - need to get; NOW](https://github.com/john-cd/rust_howto/issues/860)
// // <https://github.com/rust-lang/crates.io/issues/856>
