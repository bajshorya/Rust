
fn main() {
    let str1 = String::from("Shorya");
    let ans;
    {
        let str2=String::from("baj");
        ans= longest_string(&str1,&str2);
        println!("{}",ans);
    }
}
fn longest_string <'a, 'b>(s1: &'a String, s2: &'a String)->&'b String{
    return &s2;
}