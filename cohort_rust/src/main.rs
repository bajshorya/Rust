use std::f64::consts::PI;
use std::fs;

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn main() {
    let shape_sq = Shape::Square(5.0);
    let shape_rec = Shape::Rectangle(5.0, 10.0);
    let shape_cir = Shape::Circle(5.0);

    println!("Area of square: {}", area(&shape_sq));
    println!("Area of rectangle: {}", area(&shape_rec));
    println!("Area of circle: {}", area(&shape_cir));

    println!("Perimeter of square: {}", perimeter(&shape_sq));
    println!("Perimeter of rectangle: {}", perimeter(&shape_rec));
    println!("Perimeter of circle: {}", perimeter(&shape_cir));

    match fs::read_to_string("test.txt") {
        Ok(data) => println!("File content: {}", data),
        Err(err) => println!("Error reading file: {}", err),
    }
    let ans=find_first_a(String::from("shorya")); //option enum 
    match ans{
        None=> print!("not found"),
        Some(val)=>print!("found at index {}",val),
    }
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}

fn perimeter(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 2.0 * PI * radius,
        Shape::Rectangle(width, height) => 2.0 * (width + height),
        Shape::Square(side) => 4.0 * side,
    }
}
fn find_first_a(str:String)->Option<u32>{ //option enum
    let mut index:u32=0;
    for c in str.chars(){
        if c =='a'{
            return Some(index);
    }
    index=index+1;
    }
    None
}