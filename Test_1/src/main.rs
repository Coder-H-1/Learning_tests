use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    // Creating a random Integer value
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop{
        // Creating a variable to store user input
        let mut guess = String::new();

        // Asking for a number from user
        println!("Please guess a number : ");
        io::stdin().read_line(&mut guess).expect("Error in reading user input");

        // Changing String value to Integer for comparision
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,    
            Err(_) => continue,
        };
        // let guess: u32 = guess.trim().parse().expect("\n\n\t\tPlease enter a valid integer number.\t\t\n\n");


        // Comparing values
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Low!"),
            Ordering::Greater => println!("High!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }

}