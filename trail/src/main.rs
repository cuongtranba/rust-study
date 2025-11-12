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

fn main() {
    let circle = Circle::new(10.1);
    let area = circle.area();
    println!("Area of circle: {}", area);
}
