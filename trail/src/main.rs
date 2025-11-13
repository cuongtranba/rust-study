trait Shape {
    fn new(radius: f64) -> Self;
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

trait Creator {
    fn create(&self) {
        println!("create default")
    }
}

struct Student {
    name: String,
    age: u32,
}

struct Teacher {
    name: String,
    age: u32,
}

impl Creator for Student {
    fn create(&self) {
        println!("this is a student")
    }
}
impl Creator for Teacher {}

trait Reader {
    fn read(&self) {}
}

trait Writer {
    fn write(&self) {}
}

#[derive(Debug)]
struct ReaderWriter {}
impl Reader for ReaderWriter {
    fn read(&self) {
        println!("this is reader")
    }
}

impl Writer for ReaderWriter {
    fn write(&self) {
        println!("this is writer")
    }
}

fn new_reader_writer<T: Reader + Writer>(reader_writer: &T) {
    reader_writer.read();
    reader_writer.write();
}

fn main() {
    let circle = Circle::new(10.1);
    let area = circle.area();
    println!("Area of circle: {}", area);

    let student = Student {
        name: String::from("John"),
        age: 20,
    };
    student.create();

    let teacher = Teacher {
        name: String::from("Jane"),
        age: 30,
    };
    teacher.create();

    //---------------
    let reader_writer = ReaderWriter {};
    new_reader_writer(&reader_writer);
    println!("{:?}", reader_writer);
}
