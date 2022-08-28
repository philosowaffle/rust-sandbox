// standard library contains the io library
// which we need to interact with the terminal
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // variables are immutable by default
    // you must explicitly make a variable mutable by using `mut`
    //
    // We assign guess to a new empty string
    // :: indicates that `new` is an 'associated function'
    // an 'associated function' is a function that is implemented on a type, in
    // this case the String type.
    let mut guess = String::new();
    
    // If we had not included the io library at the top, then we could also write the below line
    // using its fully qualified reference std::io:stding
    //
    // & - this argument is a reference to a single piece of data in memory
    //   - references are immutable by default, so you must explicitly state that this reference will be mutable
    // read_line also returns a Result 
    // Result is an Enum with two variants Ok and Err
    // Ok and Err can also have data within them OK(T) and Err(E)
    match io::stdin()
        .read_line(&mut guess){
            Ok(_ok) => println!("Yay! {_ok}"),
            Err(_err) => println!("Nay! {_err}")
        }

    // test this by entering a non-integer value
    match guess.trim().parse::<f64>(){
            Ok(_ok) => println!("Yay! {_ok}"),
            Err(_err) => println!("Nay! {_err}")
        }

}
