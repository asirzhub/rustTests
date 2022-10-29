#![allow(unused)]
use std::io;
//use std::io::{Write, BufReader,  BufRead, ErrorKind};
use rand::Rng;
//use std::fs::File;
//use std::cmp::Ordering;

fn main() {
    //greeting();
    //variables();
    //random_demonstation();
    //condition();
    bossfight();
}

struct melee_weapon(){
    
};

fn bossfight(){
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p' , 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z']; 
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    const CONSONANT_COUNT: u8= 21; 
    const VOWEL_COUNT: u8 = 5; 

    // create a name for the world
    let mut kingdom_name: String = "".to_string();

    let king_name_syllables: u8 = rand::thread_rng().gen_range(3..8);
    for i in 1..king_name_syllables{
        //roll for chance to be consonant-vowel, vowel-consonant, solo vowel, or solo consonant
        let segment: u8 = rand::thread_rng().gen_range(1..11);

        //Force first syllable capital and c-v
        if(i==1){
            let rand_cons: usize = rand::thread_rng().gen_range(1..CONSONANT_COUNT).into();
            let rand_vow: usize = rand::thread_rng().gen_range(1..VOWEL_COUNT).into();
            kingdom_name.push(consonants[rand_cons].to_ascii_uppercase());
            kingdom_name.push(vowels[rand_vow]);
        }
        else if(segment<5){
            let rand_cons: usize = rand::thread_rng().gen_range(1..CONSONANT_COUNT).into();
            let rand_vow: usize = rand::thread_rng().gen_range(1..VOWEL_COUNT).into();
            kingdom_name.push(consonants[rand_cons]);
            kingdom_name.push(vowels[rand_vow]);
        }
        else if(segment<9){
            let rand_cons: usize = rand::thread_rng().gen_range(1..CONSONANT_COUNT).into();
            let rand_vow: usize = rand::thread_rng().gen_range(1..VOWEL_COUNT).into();
            kingdom_name.push(vowels[rand_vow]);
            kingdom_name.push(consonants[rand_cons]);
        }
        else if (segment==9){
            let rand_cons: usize = rand::thread_rng().gen_range(1..CONSONANT_COUNT).into();
            kingdom_name.push(consonants[rand_cons]);
        }
        else{
            let rand_vow: usize = rand::thread_rng().gen_range(1..VOWEL_COUNT).into();
            kingdom_name.push(vowels[rand_vow]);
        }
    }

    // create a name for the world
    let mut dragon_name: String = "".to_string();

    let dragon_name_syllables: u8 = rand::thread_rng().gen_range(3..8);
    for i in 1..dragon_name_syllables{
        //roll for chance to be consonant-vowel, vowel-consonant, solo vowel, or solo consonant
        let segment: u8 = rand::thread_rng().gen_range(1..11);

        //Force first syllable capital and c-v
        if(i==1){
            let rand_cons: usize = rand::thread_rng().gen_range(1..CONSONANT_COUNT).into();
            let rand_vow: usize = rand::thread_rng().gen_range(1..VOWEL_COUNT).into();
            dragon_name.push(consonants[rand_cons].to_ascii_uppercase());
            dragon_name.push(vowels[rand_vow]);
        }
        else if(segment<5){
            let rand_cons: usize = rand::thread_rng().gen_range(1..CONSONANT_COUNT).into();
            let rand_vow: usize = rand::thread_rng().gen_range(1..VOWEL_COUNT).into();
            dragon_name.push(consonants[rand_cons]);
            dragon_name.push(vowels[rand_vow]);
        }
        else if(segment<9){
            let rand_cons: usize = rand::thread_rng().gen_range(1..CONSONANT_COUNT).into();
            let rand_vow: usize = rand::thread_rng().gen_range(1..VOWEL_COUNT).into();
            dragon_name.push(vowels[rand_vow]);
            dragon_name.push(consonants[rand_cons]);
        }
        else if (segment==9){
            let rand_cons: usize = rand::thread_rng().gen_range(1..CONSONANT_COUNT).into();
            dragon_name.push(consonants[rand_cons]);
        }
        else{
            let rand_vow: usize = rand::thread_rng().gen_range(1..VOWEL_COUNT).into();
            dragon_name.push(vowels[rand_vow]);
        }
    }
    
    //intro
    println!("You are the best knight from the Kingdom Of {}! What is your name?", kingdom_name);
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("no input?  ");
    name = name.trim_end().to_string();

    println!("Well then, Knight {}! As the best warrior of {}, you have been selected to take down the Great Dragon known as {}", name, kingdom_name, dragon_name);
    

}


fn condition(){
    let num_of_cars = rand::thread_rng().gen_range(0..3);
    if(num_of_cars > 1){
        println!("You have too many cars! {} is more than one.", num_of_cars);
    }
    else{
        println!("Nice. You have {} car(s), so you don't have too many cars", num_of_cars);
    }
}

fn random_demonstation(){
    let mut rand_int = rand::thread_rng().gen_range(1..101);
    let mut rand_float = (rand_int as f32)/100.0;

    println!("Random integer: {} and as a float: {}", rand_int, rand_float);
}

fn variables(){
    const PI: f32 = 3.141592653589;
    const YEAR: f32 = 2022.0;
    const FINAL: f32 = PI * YEAR;
    
    // use ({}, variable) or {variable}
    println!("What is this year times pi?");
    print!("{}", FINAL);
    println!("or, in another format:{FINAL}");

    // name, age, right-handed, eye colour
    #[derive(Debug)]
    enum Colors {Red, Blue, Green, Yellow, Orange, Purple, White, Black, Brown}
    let about_me : (&str, i32, bool, Colors) = ("Zaki", 20, true, Colors::Brown);
    println!("Name: {} Age: {} Right Handed: {} Eye Colour: {:?}", 
        about_me.0, about_me.1, about_me.2, about_me.3);
}

fn greeting(){
    println!("What's your name?");
    //mut keyword: mutable variable (default immutable so this needs to be specific)
    let mut name: String = String::new();
    let greeting: &str = "This is a greeting message";
    let name_acknowledgement: &str = "So your name is";

    io::stdin().read_line(&mut name).expect("no input?");
    println!("{}! {} {}.", greeting, name_acknowledgement, name.trim_end());
}
