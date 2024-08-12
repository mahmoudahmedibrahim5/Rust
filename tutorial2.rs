use std::io;
use rand::Rng;

fn main() {
    println!("Guess The number from 0 to 9");
    let secret_number:u8 = rand::thread_rng().gen_range(0..=9);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Faile to input");

    let guessed_number:u8 = input.trim().parse().unwrap();

    if secret_number == guessed_number{
        println!("You Won, the secret number was {}", secret_number);
    }
    else{
        println!("You Lost, the secret number was {}", secret_number);
    }
}
