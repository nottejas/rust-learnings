use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    
    let sec_num = rand::thread_rng().gen_range(1..=100);

    println!("The sec num is: {sec_num}");

    println!("Input your guess");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed: {guess}");

}


