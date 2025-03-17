use std::f64::consts::PI;
use std::f64;

struct Circle {
    radius: f64
}

impl Circle {
    fn area(&mut self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Rectangle {
    length: f64,
    breadth: f64
}

impl Rectangle {
    fn area(&mut self) -> f64 {
        self.length * self.breadth
    }
}

struct Square {
    side: f64
}

impl Square {
    fn area(&mut self) -> f64 {
        self.side * self.side
    }
}

struct Triangle {
    base: f64,
    height: f64
}

impl Triangle {
    fn area(&mut self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Parallelogram {
    base: f64,
    height: f64
}

impl Parallelogram {
    fn area(&mut self) -> f64 {
        self.base * self.height
    }
}

struct Trapezium {
    base_1: f64,
    base_2: f64,
    height: f64
}

impl Trapezium {
    fn area(&mut self) -> f64 {
        0.5 * (self.base_1 + self.base_2) * self.height
    }
}

fn main() {
    let mut circle = Circle { radius: 5.0 };
    println!("Circle Area: {}", circle.area());

    let mut rectangle = Rectangle { length: 10.0, breadth: 5.0 };
    println!("Rectangle Area: {}", rectangle.area());

    let mut square = Square { side: 4.0 };
    println!("Square Area: {}", square.area());

    let mut triangle = Triangle { base: 6.0, height: 3.0 };
    println!("Triangle Area: {}", triangle.area());

    let mut parallelogram = Parallelogram { base: 7.0, height: 4.0 };
    println!("Parallelogram Area: {}", parallelogram.area());

    let mut trapezium = Trapezium { base_1: 8.0, base_2: 6.0, height: 5.0 };
    println!("Trapezium Area: {}", trapezium.area());
}