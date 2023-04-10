use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    println!("Guess the number\n");
    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("{secret}");
    io::stdin().read_line(&mut guess).expect("Failed to guess");

    let guess: u32 = guess.trim().parse().expect("please type a number");
    match guess.cmp(&secret) {
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Equal"),
        Ordering::Less => println!("Too small"),
    }

    println!(" You guessed {guess}");
}