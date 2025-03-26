#![allow(unused)]           // Suppress unused variable warnings

// traits (Interfaces)

// Bad example - the shared traits don't make sense

fn main(){
    const PI: f32 = 3.141592;

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {length: f32, width: f32};
    struct Circle {length: f32, width: f32};

    impl Shape for Rectangle{
        fn new(length: f32, width: f32) -> Rectangle {
            // return Rectangle{length, width};
            Rectangle{length, width}             // Implicit return better style
        }
        fn area(&self) -> f32 {
            // return self.length * self.width;
            self.length * self.width             // Implicit return better style
        }
    }

    impl Shape for Circle{
        // What is the length of a circle??
        fn new(length: f32, width: f32) -> Circle {
            Circle{length, width}
        }
        fn area(&self) -> f32 {
            PI * (self.width / 2.0) .powf(2.0)
        }
    }

    let rec_01: Rectangle = Shape::new(10.0, 5.0);
    let cir_01: Circle = Shape::new(10.0, 5.0);

    println!("Rec area: {}", rec_01.area());
    println!("Cir area: {}", cir_01.area());
}