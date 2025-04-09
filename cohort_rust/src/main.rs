use std::f64::consts::PI;
enum Shape{
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn main() {
    let shpae_sq= Shape::Square(5.0);
    let shpae_rec= Shape::Rectangle(5.0, 10.0);
    let shpae_cir= Shape::Circle(5.0);
    println!("Area of square: {}", area(shpae_sq));
    println!("Area of rectangle: {}", area(shpae_rec));
    println!("Area of circle: {}", area(shpae_cir));
    let shape_sq= Shape::Square(5.0);
    let shape_rec= Shape::Rectangle(5.0, 10.0);
    let shape_cir= Shape::Circle(5.0);
    println!("Perimeter of square: {}", perimeter(shape_sq));
    println!("Perimeter of rectangle: {}", perimeter(shape_rec));
    println!("Perimeter of circle: {}", perimeter(shape_cir));
    
}
fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}
fn perimeter(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 2.0 * PI * radius,
        Shape::Rectangle(width, height) => 2.0 * (width + height),
        Shape::Square(side) => 4.0 * side,
    }
}