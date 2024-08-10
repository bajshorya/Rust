mod rect;
mod shape;
use shape::{Shape, calculate_area};
use rect::Rect;

struct User {
    age: u32,
    name: String,
    happy: bool,
   
}



fn main() {
   let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    // Calculate and print the areas
    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));




    let user1 = User {
        age: 20,
        name: String::from("Shorya"),
        happy: true,
        
    };
    println!("User {} whose age is {} is happy {}", user1.name ,user1.age ,user1.happy);
    let rect = Rect {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {}", rect.area());
    println!("The perimeter of the rectangle is {}", rect.perimeter());
}
