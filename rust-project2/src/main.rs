struct User {
    age: u32,
    name: String,
    happy: bool,
   
}
struct Rect {
   width: u32,
   height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
         self.width * self.height
    }
    fn perimeter(&self) -> u32 {
         2*(self.width + self.height)
    }
}

fn main() {
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
