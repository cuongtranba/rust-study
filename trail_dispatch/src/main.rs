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

trait EmailSender {
    fn send(&self);
}

struct SMTPMailer {
    email: String,
}

struct BasicMailer {
    email: String,
}

impl EmailSender for SMTPMailer {
    fn send(&self) {
        println!("Sending email via SMTP to {}", self.email);
    }
}

impl EmailSender for BasicMailer {
    fn send(&self) {
        println!("Sending email via BasicMailer to {}", self.email);
    }
}

struct WebcomeService {
    mail_sender: Box<dyn EmailSender>,
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
            // Can not do this because Screen need Button type, and rust need to know the size type in compile time
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

    let mut webcome_service = WebcomeService {
        mail_sender: Box::new(BasicMailer {
            email: String::from("example@example.com"),
        }),
    };

    webcome_service.mail_sender.send();
    webcome_service.mail_sender = Box::new(SMTPMailer {
        email: String::from("example@example.com"),
    });
}
