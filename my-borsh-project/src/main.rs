use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

fn main() {
    let user = User {
        username: String::from("Shorya"),
        password: String::from("123456"),
    };

    let mut v= Vec::new();//create a vector to store the serialized data 
    let ans = user.serialize(&mut v); //serialize the data and store it in the vector
    match ans {
        Ok(()) => println!("Serialized: {:?}", v), //if the data is serialized successfully, print the vector
        Err(e) => println!("Error: {:?}", e), //if the data is not serialized successfully, print the error
    }
    
}
