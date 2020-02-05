use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Gess the number!");

  let secret_nb = rand::thread_rng().gen_range(1,101);

  println!("The secret number is {}", secret_nb);

  let mut guess = String::new();

  io::stdin().read_line(&mut guess)
    .expect("Failed to read line!");
  
  println!("You gessed : {}", guess);

  match guess.cmp(&secret_nb) {
    Ordering::Less => println!("Too Smalli"),
  }
}
