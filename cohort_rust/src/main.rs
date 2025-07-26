use std::f64::consts::PI; //PI is a constant in the std library
use std::fs; //fs is a module in the std library

enum Shape { //enum is a collection of related values 
    Circle(f64), 
    Rectangle(f64, f64),
    Square(f64),
}

fn main() {
    let shape_sq = Shape::Square(5.0); //shape_sq is a variable of type Shape 
    let shape_rec = Shape::Rectangle(5.0, 10.0); //shape_rec is a variable of type Shape
    let shape_cir = Shape::Circle(5.0); //shape_cir is a variable of type Shape 

    println!("Area of square: {}", area(&shape_sq)); //area is a function that takes a reference to a Shape and returns a f64
    println!("Area of rectangle: {}", area(&shape_rec)); //area is a function that takes a reference to a Shape and returns a f64
    println!("Area of circle: {}", area(&shape_cir)); //area is a function that takes a reference to a Shape and returns a f64

    println!("Perimeter of square: {}", perimeter(&shape_sq)); //perimeter is a function that takes a reference to a Shape and returns a f64
    println!("Perimeter of rectangle: {}", perimeter(&shape_rec)); //perimeter is a function that takes a reference to a Shape and returns a f64
    println!("Perimeter of circle: {}", perimeter(&shape_cir)); //perimeter is a function that takes a reference to a Shape and returns a f64

    match fs::read_to_string("test.txt") { //match is a keyword that is used to match a value to a pattern
        Ok(data) => println!("File content: {}", data), //Ok is a variant of the Result enum
        Err(err) => println!("Error reading file: {}", err), //Err is a variant of the Result enum
    }
    let ans = find_first_a(String::from("shorya")); //option enum
    match ans { //match is a keyword that is used to match a value to a pattern
        None => print!("not found"), //None is a variant of the Option enum
        Some(val) => print!("found at index {}", val), //Some is a variant of the Option enum
    }
}

fn area(shape: &Shape) -> f64 { //area is a function that takes a reference to a Shape and returns a f64
    match shape { //match is a keyword that is used to match a value to a pattern
        Shape::Circle(radius) => PI * radius * radius, //PI is a constant in the std library
        Shape::Rectangle(width, height) => width * height, //width and height are parameters of the Rectangle struct
        Shape::Square(side) => side * side, //side is a parameter of the Square struct
    }
}

fn perimeter(shape: &Shape) -> f64 { //perimeter is a function that takes a reference to a Shape and returns a f64
    match shape { //match is a keyword that is used to match a value to a pattern
        Shape::Circle(radius) => 2.0 * PI * radius, //PI is a constant in the std library
        Shape::Rectangle(width, height) => 2.0 * (width + height), //width and height are parameters of the Rectangle struct
        Shape::Square(side) => 4.0 * side, //side is a parameter of the Square struct
    }
}
fn find_first_a(str: String) -> Option<u32> { //find_first_a is a function that takes a String and returns an Option<u32>
    //option enum
    let mut index: u32 = 0; //index is a variable of type u32
    for c in str.chars() { //for is a keyword that is used to iterate over a collection
        if c == 'a' {
            return Some(index); //Some is a variant of the Option enum
        }
        index = index + 1; //index is incremented by 1
    }
    None //None is a variant of the Option enum
}
