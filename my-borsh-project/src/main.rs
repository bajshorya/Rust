use borsh::{BorshSerialize, BorshDeserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    fn new(value: i32) -> Self {
        Self { value }
    }
    
    fn increment(&mut self) {
        self.value += 1;
    }
    
    fn decrement(&mut self) {
        self.value -= 1;
    }
}

fn read_counter_from_file(file_path: &Path) -> std::io::Result<Counter> { //read_counter_from_file is a function that reads the counter from the file and returns a Counter 
    let mut file = File::open(file_path)?; //open the file 
    let mut buffer = Vec::new(); //create a new vector to store the data of the file
    file.read_to_end(&mut buffer)?; //read the data from the file and store it in the vector 
    Counter::try_from_slice(&buffer).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)) //try to convert the vector to a Counter and return the Counter 
}

fn write_counter_to_file(file_path: &Path, counter: &Counter) -> std::io::Result<()> { //write_counter_to_file is a function that writes the counter to the file and returns a Result
    let mut file = File::create(file_path)?; //create a new file
    let mut serialized = Vec::new(); //create a new vector to store the data of the counter
    counter.serialize(&mut serialized) //serialize the counter and store it in the vector
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?; //if there is an error, return an error
    file.write_all(&serialized)?; //write the data to the file
    Ok(()) //return Ok if there is no error 
}

fn increment(file_path: &str) -> std::io::Result<()> { //increment is a function that increments the counter and returns a Result
    let path = Path::new(file_path); //create a new path
    let mut counter = read_counter_from_file(path)?; //read the counter from the file
    counter.increment(); //increment the counter
    write_counter_to_file(path, &counter) //write the counter to the file
}

fn decrement(file_path: &str) -> std::io::Result<()> { //decrement is a function that decrements the counter and returns a Result   
    let path = Path::new(file_path); //create a new path
    let mut counter = read_counter_from_file(path)?; //read the counter from the file
    counter.decrement(); //decrement the counter
    write_counter_to_file(path, &counter) //write the counter to the file
}

fn main() -> std::io::Result<()> { //main is the entry point of the program
    // Counter operations
    let file_path = "a.bin"; //create a new file path
    if !Path::new(file_path).exists() { //if the file does not exist
        let initial_counter = Counter::new(0); //create a new counter
        write_counter_to_file(Path::new(file_path), &initial_counter)?; //write the counter to the file
    }

    increment(file_path)?; //increment the counter
    decrement(file_path)?; //decrement the counter
    increment(file_path)?; //increment the counter
    increment(file_path)?; //increment the counter

    let counter = read_counter_from_file(Path::new(file_path))?; //read the counter from the file
    println!("Final counter value: {}", counter.value); //print the final counter value

    // User serialization demo
    let user = User {
        username: String::from("Shorya"), //create a new user
        password: String::from("123456"), //create a new password
    };

    let mut v = Vec::new(); //create a new vector
    user.serialize(&mut v) //serialize the user and store it in the vector
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?; //if there is an error, return an error
    println!("Serialized User: {:?}", v); //print the serialized user

    let u = User::try_from_slice(&v) //try to deserialize the user from the vector
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?; //if there is an error, return an error
    println!("Deserialized User: {:?}", u); //print the deserialized user

    Ok(()) //return Ok if there is no error
}