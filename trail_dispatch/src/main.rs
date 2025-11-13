pub trait Draw {
    fn draw(&self);
}

struct Button {
    text: String,
}

struct TextField {
    text: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button with text: {}", self.text);
    }
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing text field with text: {}", self.text);
    }
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}
pub struct ScreenDynamicDispatch {
    pub components: Vec<Box<dyn Draw>>,
}

impl<T: Draw> Screen<T> {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

impl ScreenDynamicDispatch {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Button {
                text: String::from("Click me 1"),
            },
            Button {
                text: String::from("Click me 2"),
            },
            // Can not do this because Screen need Button type
            // TextField {
            //     text: String::from("Enter text"),
            // },
        ],
    };
    println!("static dispatch:");
    screen.run();

    let dynamic_screen = ScreenDynamicDispatch {
        components: vec![
            Box::new(Button {
                text: String::from("Click me 3"),
            }),
            Box::new(Button {
                text: String::from("Click me 4"),
            }),
            Box::new(TextField {
                text: String::from("Enter text"),
            }),
        ],
    };
    println!("dynamic dispatch:");
    dynamic_screen.run();
}
