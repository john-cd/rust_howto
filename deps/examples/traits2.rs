trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // <-- default implementation
        format!("(Read more from {}...)", self.summarize_author()) // <-- can call a non-default method
    }
}

fn main() {}
