use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Gess the number!");

    let secret_nb = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_nb);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        println!("You gessed : {}", guess);

        let guess: u32 = guess.trim().parse().expect("Please enter a number");

        match guess.cmp(&secret_nb) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Got it");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
        }
    }
}
