#![allow(dead_code)]
// ANCHOR: example
struct CustomStruct {
    data: String,
}

impl Drop for CustomStruct {
    // This is the only method required by the Drop trait. It takes a mutable
    // reference to self. The code inside this method will be executed when
    // an instance of CustomStruct is dropped.
    fn drop(&mut self) {
        println!("Dropping CustomStruct with data `{}`!", self.data);
    }
}

fn main() {
    let _c = CustomStruct {
        data: String::from("my data"),
    };
    let _d = CustomStruct {
        data: String::from("another data"),
    };

    println!("CustomStructs created.");
    // c and d will be dropped automatically when they go out of scope here.

    let e = CustomStruct {
        data: String::from("explicit drop example"),
    };
    println!("CustomStruct 'e' created.");
    // We can explicitly drop a value using `std::mem::drop`, but usually,
    // you let Rust handle it automatically.
    drop(e);
    println!("CustomStruct 'e' dropped before end of main.");

    println!("End of main.");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
