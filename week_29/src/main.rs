use chrono::prelude::*;
fn main() {
    let utc: DateTime<Utc> = Utc::now(); 
    let local = Local::now(); 


    println!("Hello, world!, the current time is :{} and {}",utc,local);
}
