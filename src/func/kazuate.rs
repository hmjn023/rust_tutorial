use rand::Rng;
use std::io;

pub fn kazuate() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guesseed {}", guess);

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too large"),
            std::cmp::Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }
}
