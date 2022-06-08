use rand::Rng;
use std::cmp::Ordering;
use std::io; // prelude // trait

fn main() {
    println!("~~Guessing Game~~");

    let secret_number = rand::thread_rng().gen_range(1..101); // i32 u32 i64

    loop {
        println!("Please input a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read input!"); // io::Result Ok/Err

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("is smaller"),
            Ordering::Greater => println!("is bigger"),
            Ordering::Equal => {
                println!("~~win win win~~");
                break;
            }
        }
    }
}
