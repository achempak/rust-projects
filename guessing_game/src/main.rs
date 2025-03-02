// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new(); // guess is mutable
    io::stdin()
        .read_line(&mut guess) // stores input in guess and returns a Result
        .expect("Failed to read line"); // if Result is Err, then display this message
    print!("You guessed: {}", guess);
}
