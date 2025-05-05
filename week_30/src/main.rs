macro_rules! say_hello{
    ()=>{
        println!("zzzzzz");
    }
}
fn main() {
    let v = vec!(1,2,); //declarative macro 
    println!("{:?}",v);
    say_hello!();
}
