use front_of_house::hosting;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

#[test]
fn test() {
    eat_at_restaurant();
}
