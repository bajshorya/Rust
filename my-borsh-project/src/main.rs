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

fn write_counter_to_file(file_path: &Path, counter: &Counter) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    let mut serialized = Vec::new();
    counter.serialize(&mut serialized)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    file.write_all(&serialized)?;
    Ok(())
}

fn increment(file_path: &str) -> std::io::Result<()> {
    let path = Path::new(file_path);
    let mut counter = read_counter_from_file(path)?;
    counter.increment();
    write_counter_to_file(path, &counter)
}

fn decrement(file_path: &str) -> std::io::Result<()> {
    let path = Path::new(file_path);
    let mut counter = read_counter_from_file(path)?;
    counter.decrement();
    write_counter_to_file(path, &counter)
}

fn main() -> std::io::Result<()> {
    // Counter operations
    let file_path = "a.bin";
    if !Path::new(file_path).exists() {
        let initial_counter = Counter::new(0);
        write_counter_to_file(Path::new(file_path), &initial_counter)?;
    }

    increment(file_path)?;
    decrement(file_path)?;
    increment(file_path)?;
    increment(file_path)?;

    let counter = read_counter_from_file(Path::new(file_path))?;
    println!("Final counter value: {}", counter.value);

    // User serialization demo
    let user = User {
        username: String::from("Shorya"),
        password: String::from("123456"),
    };

    let mut v = Vec::new();
    user.serialize(&mut v)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    println!("Serialized User: {:?}", v);

    let u = User::try_from_slice(&v)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    println!("Deserialized User: {:?}", u);

    Ok(())
}