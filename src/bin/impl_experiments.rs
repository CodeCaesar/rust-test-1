use std::f32::consts::PI;

struct Circle {
    radius: f32
}

impl Circle {
    fn circumference(&self) -> f32 {
        2.0 * PI * self.radius
    }
}

fn main() {
    let circle: Circle = Circle { radius: 1.5 };

    println!("Circle with radius of {}\nAnd circumference of {}", circle.radius, circle.circumference());
}