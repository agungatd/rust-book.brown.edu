use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut counter: u32 = 0;
    // println!("The secret number is: {secret_number}");
    
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");

        counter += 1;
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, my friend!"),
            Ordering::Greater => println!("It's too big, ughh!"),
            Ordering::Equal => {
                println!("That's correct, Congrats!!!");
                println!("You successfully guess the number in {counter} steps");
                break;
                todo!("implement grade from guessing steps")
            },
        }
    }

}
