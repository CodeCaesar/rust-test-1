use std::f32::consts::PI;

struct Circle {
    radius: f32
}

impl Circle {
    fn circumference(&self) -> f32 {
        2.0 * PI * self.radius
    }

    fn diameter(&self) -> f32 {
        self.radius * 2.0
    }

    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }
}

fn main() {
    let circle: Circle = Circle { radius: 1.5 };

    println!("Circle with radius of {}\nAnd circumference of {}\nAnd diameter of {}\nAnd area of {}", circle.radius, circle.circumference(), circle.diameter(), circle.area());
}