// use std::{fmt::format,path::Display};
macro_rules! say_hello{
    ()=>{
        println!("zzzzzz");
    }
}
#[derive(Debug)]
struct User{
    username:String,
    name:String,
    password:u32
}
// impl Debug for User{
//     fn fmt(&self, f: &mut Formatter<'_>) → std:: fmt:: Result {
//         write! (f, "users username is {}", self. username)
//     }
// }
//instead of making this Debug treait for User we used the Debug macro above the USer Trait !!!!


fn main() {
    let u = User{
        username:String::from("zzzz"),
        name:String::from("aaa"),
        password:2
    };
    println!("{:?}",u);
    let v = vec!(1,2,); //declarative macro 
    println!("{:?}",v);
    say_hello!();
}
