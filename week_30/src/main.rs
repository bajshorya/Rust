// use std::{fmt::format,path::Display};

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]

// macro_rules! say_hello{
//     ()=>{
//         println!("zzzzzz");
//     }
// }
// #[derive(Debug)]
struct User{
    username:String,
    name:String
    // password:u32
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
        name:String::from("aaa")
        // password:2
    };
    let serialized_user=serde_json::to_string(&u);
    match serialized_user{
        Ok(str)=>print!("{}",str),
        Err(_)=>print!("Error!!!!")
    }
    let s = String::from("{\"username\": \"user@\", \"name\": \"userOne\"}");
    let ans: Result<User, serde_json::Error> = serde_json::from_str(&s);

    match ans {
        Ok(user) => println!("{:?}", user),
        Err(e) => println!("Error parsing JSON: {:?}", e),
    }
    // println!("{:?}",u);
    // let v = vec!(1,2,); //declarative macro 
    // println!("{:?}",v);
    // say_hello!();
}
