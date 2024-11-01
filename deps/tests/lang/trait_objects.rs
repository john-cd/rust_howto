// ANCHOR: example
trait Draw {
    fn draw(&self);
}

struct Button;

impl Draw for Button {
    fn draw(&self) {
        println!("Button");
    }
}

struct Text;

impl Draw for Text {
    fn draw(&self) {
        println!("Text");
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>, // <-- trait object
}

impl Screen {
    fn new() -> Self {
        Screen {
            components: vec![Box::new(Button), Box::new(Text), Box::new(Text)],
        }
    }

    fn run(&self) {
        for component in self.components.iter() {
            // The purpose of trait objects is to permit "late binding" of
            // methods. Calling a method on a trait object results
            // in virtual dispatch at runtime. Here, `components` is
            // a mix of `Button` and `Text` structs.
            component.draw();
        }
    }
}

fn main() {
    let s = Screen::new();
    s.run();
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
