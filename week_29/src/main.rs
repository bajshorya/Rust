#[derive(Copy,Clone)]

struct Rect<T>{ // struct to generics 
    width : T,
    height : T
} 


impl <T: std::ops::Mul<Output=T>+Copy> Rect<T>{
    fn area(&self) -> T{
        return self.width * self.height;
    }
}

fn main(){
    // display_element(1,2);
    // display_element(String::from("harkirat"),String::from("Shorya"));
    let r = Rect{
        width:10,
        height:20
    };
    print!("area is:{}",r.area());
}

// impl<T> Rect<T> where T: Mul<Output =T> +Copy, {
//     fn area(&self) -> T{
//         return self.width * self.height;
//     }
// }


// fn display_element<T: std::fmt::Display>(a:T,b:T){
//     println!("{}",a);
//     println!("{}",b);
// }
// impl Rect<i32>{
//     fn area(&self) -> i32{
//         return self.width* self.height;
//     }
// }
// use chrono::prelude::*;
// fn main() {
//     let utc: DateTime<Utc> = Utc::now(); 
//     let local = Local::now(); 


//     println!("Hello, world!, the current time is :{} and {}",utc,local);
// }

// struct User{
//     username:String
// }