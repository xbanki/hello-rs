use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    let rng = rand::thread_rng().gen_range(1..=100);
    println!("Time to guess a number between 1 -> 100!\n");
    loop {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => match guess.trim() {
                "q" => break,
                "Q" => break,
                _ => match guess.trim().parse::<u32>() {
                    Ok(result) => match result.cmp(&rng) {
                        Ordering::Greater => println!("Too big."),
                        Ordering::Equal => {
                            println!("Correct! You win.");
                            break;
                        }
                        Ordering::Less => println!("Too small."),
                    },
                    Err(_) => continue,
                },
            },
            _ => {}
        }
    }
}
