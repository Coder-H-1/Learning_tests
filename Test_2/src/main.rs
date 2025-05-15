fn factorial(x:i32) -> i32{
    if x < 2 {
        return 1;
    }
    else{
        return x * factorial( x-1 ) ;
    }
}

use std::io;

fn Even_Odd(){
    println!("Enter a number : ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Error in reading line!");
    let number:i32 = number.trim().parse().expect("Not a number");
    let temp_list = [ "Even" , "Odd" ];
    let num = number % 2;
    println!("{}" , temp_list[ num ] );
}

fn main(){
    Even_Odd();
}