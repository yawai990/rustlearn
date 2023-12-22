use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    let secret_number = rand::thread_rng().gen_range(0..=10);

    println!("SECRET NUMBER IS {secret_number}");

    let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("your guess number is {guess}");


    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}