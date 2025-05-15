// A Guessing Game
use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main(){
    // Secret number
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    
    loop{
        println!("Enter a number :");
        // Taking an natural number from user
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error in User Input");

        // converting User inputted string into unsigned 32-bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("\t\t\tLOW!"),
            Ordering::Greater => println!("\t\t\tHigh!"),
            Ordering::Equal => {
                println!("\t\t\tYou Win!");
                break;
            }

        };
    }
    
}