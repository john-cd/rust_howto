// TODO
#![allow(dead_code)]

trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
        // The default implementation can call a non-default
        // (abstract) method
    }
}

struct Blog {
    author: String,
}

impl Summary for Blog {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

#[test]
fn test() {
    let blog = Blog {
        author: "ferris".into(),
    };
    println!("{}", blog.summarize());
}
