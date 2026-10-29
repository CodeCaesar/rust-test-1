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

struct RightAngleTriangle {
    height: f32,
    length: f32
}

impl RightAngleTriangle {
    fn hypotenuse(&self) -> f32 {
        let square_sum: f32 = self.height * self.height + self.length * self.length;
        square_sum.sqrt()
    }

    fn perimeter(&self) -> f32 {
        self.height + self.length + self.hypotenuse()
    }

    fn area(&self) -> f32 {
        self.height * self.length / 2.0
    }
}

struct Rectangle {
    height: f32,
    length: f32
}

impl Rectangle {
    fn perimeter(&self) -> f32 {
        2.0 * (self.height + self.length)
    }

    fn area(&self) -> f32 {
        self.height * self.length
    }

    fn is_square(&self) -> bool {
        if self.height == self.length {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let circle: Circle = Circle { radius: 1.5 };
    let ra_traingle: RightAngleTriangle = RightAngleTriangle { height: 3.8, length: 2.3 };
    let rectangle: Rectangle = Rectangle { height: 10.3, length: 4.5 };
    let square: Rectangle = Rectangle { height: 2.5, length: 2.5 };

    println!("Circle with radius of {}\nAnd circumference of {}\nAnd diameter of {}\nAnd area of {}", circle.radius, circle.circumference(), circle.diameter(), circle.area());
    println!("Right-Angle Triangle with height of {} and length of {}\nAnd hypotenuse of {}\nAnd perimeter of {}\nAnd area of {}", 
        ra_traingle.height, ra_traingle.length, ra_traingle.hypotenuse(), ra_traingle.perimeter(), ra_traingle.area());
    
    println!("Rectangle with height of {} and length of {}\nAnd perimeter of {}\nAnd area of {}\nAnd is square: {}", 
        rectangle.height, rectangle.length, rectangle.perimeter(), rectangle.area(), rectangle.is_square());
    println!("Square with height of {} and length of {}\nAnd perimeter of {}\nAnd area of {}\nAnd is square: {}", 
        square.height, square.length, square.perimeter(), square.area(), square.is_square());
}