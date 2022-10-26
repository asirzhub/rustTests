use std::io;
//use std::io::{Write, BufReader,  BufRead, ErrorKind};
//use rand::Rng;
//use std::fs::File;
//use std::cmp::Ordering;

fn main() {
    println!("What's your name?");
    //mut keyword: mutable variable (default immutable so this needs to be specific)
    let mut name: String = String::new();
    let greeting: &str = "This is a greeting message";
    let name_acknowledgement: &str = "So your name is";

    io::stdin().read_line(&mut name).expect("no input?");
    println!("{}! {} {}.", greeting, name_acknowledgement, name.trim_end());
}
