use std::fs::File;
use std::io::{self, Read, Write};

struct Person {
    name: String,
    SID: u32,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What's your student ID? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let SID = buffer.trim().parse().unwrap();

    let person = Person { name, SID };
    println!("Hi {}, your student ID is {}!", person.name, person.SID);
}

struct Config {
    name: String,
    SID: u32,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let SID = lines.next().unwrap().parse().unwrap();

        Config { name, SID }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("Name: {}", config.name);
    println!("Student ID: {}", config.SID);
}

fn main() {
    reading_from_console();
    reading_from_file();
}

