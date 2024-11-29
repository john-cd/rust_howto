#![allow(dead_code)]
// ANCHOR: example
// TODO P1 fix - the API no longer returns a crate_id - need to get
// version_id then join version_id and Version.id ro retrieve crate
// name Consider a simpler API for example purposes

// use anyhow::Result;
// use serde::Deserialize;

// // Structs used to deserialize the JSON produced by the API

// #[derive(Deserialize)]
// struct ApiResponse {
//     dependencies: Vec<Dependency>,
//     meta: Meta,
//     versions: Vec<Version>,
// }

// // https://github.com/rust-lang/crates.io/issues/856
// #[derive(Deserialize)]
// struct Dependency {
//     version_id: String,
// }

// #[derive(Deserialize)]
// struct Meta {
//     total: u32,
// }

// #[derive(Deserialize)]
// struct Version {
//     id: String,
//     #[serde(alias = "crate")]
//     crate_id: String,
// }

// // Main struct

// struct ReverseDependencies {
//     crate_id: String,
//     dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
//     client: reqwest::blocking::Client,
//     page: u32,
//     per_page: u32,
//     total: u32,
// }

// impl ReverseDependencies {
//     fn of(crate_id: &str) -> Result<Self> {
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

//     fn try_next(&mut self) -> Result<Option<Dependency>> {
//         if let Some(dep) = self.dependencies.next() {
//             return Ok(Some(dep));
//         }

//         if self.page > 0 && self.page * self.per_page >= self.total
// {             return Ok(None);
//         }

//         self.page += 1;
//         let url = format!(
//             "https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
//             self.crate_id, self.page, self.per_page
//         );
//         println!("Calling {}", url);
//         let resp = self.client.get(url).send()?;
//         //println!("{:#?}", resp);
//         //let text = resp.text()?;
//         //println!("{:#?}", text);
//         let json = resp.json::<ApiResponse>()?;
//         self.dependencies = json.dependencies.into_iter();
//         self.total = json.meta.total;
//         Ok(self.dependencies.next())
//     }
// }

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
//         println!("reverse dependency: {}", dep?.crate_id);
//     }
//     Ok(())
// }

fn main() -> anyhow::Result<()> {
    Ok(())
}
// ANCHOR_END: example

// Requires network access
#[test]
fn test() -> anyhow::Result<()> {
    let res = main();
    println!("{:?}", res);
    res?;
    Ok(())
}
