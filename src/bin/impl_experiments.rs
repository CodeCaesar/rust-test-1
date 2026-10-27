use std::f32::consts::PI;

struct Circle {
    radius: f32
}

fn main() {
    let circle: Circle = Circle { radius: 1.5 };

    let circumference: f32 = 2.0 * PI * circle.radius;

    println!("Circle with radius of {}\nAnd circumference of {}", circle.radius, circumference);
}