trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
        // The default implementation can call a non-default (abstract) method
    }
}

fn main() {}
