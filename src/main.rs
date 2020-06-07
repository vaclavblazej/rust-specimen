use rand::Rng;
use std::io::{self, Read};

fn main() {
    let mut rng = rand::thread_rng();
    let n:i32 = rng.gen();
    let m = ((n%100)+100)%100;
    println!("{}", m);
    let mut guess = -1;
    while guess != m {
        print!("guess: ");
        if guess > m {
            println!("lower");
        } else if guess < m {
            println!("higher");
        }

        let mut input = Input::Standard(io::stdin());
        guess = input!();
    }
    println!("Correct!");
}
