#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let c = Circle {
        radius: 2.0
    };
    println!("Area of {:?} is {}", c, c.area());
}