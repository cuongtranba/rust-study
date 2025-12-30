struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        User { name, age }
    }
}

struct Human {
    name: String,
    age: u32,
}

impl Human {
    fn new(name: String, age: u32) -> Self {
        Human { name, age }
    }
}

impl From<&User> for Human {
    fn from(user: &User) -> Self {
        Human {
            name: user.name.clone(),
            age: user.age,
        }
    }
}

fn convert_to_human(user: &User) -> Human {
    user.into()
}

fn main() {
    let user = User::new("John".to_string(), 30);
    let human = Human::from(&user);
    println!("Hello, {}, age: {}!", human.name, human.age);
    let human = convert_to_human(&user);
    println!("Hello, {}, age: {}!", human.name, human.age);
}
