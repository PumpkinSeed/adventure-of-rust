// get the std::io it's attach a second prelude
// here we can use an umber of useful io-related things
use std::io;

// the main function is the entry point of the program
fn main() {
    
    // the println!() is a macro that prints a string to the screen
    println!("Guess the number!");
    println!("Please input your guess.");

    // let creates a new variable which is immutable by default 
    // the let mut means it will be mutable
    // String is a string type provided by the standard library
    // ::new() it's an associated function
    let mut guess = String::new();

    // ::stdin() calls an associated function in io prelude
    // read_line takes a &mut String as paramter and returns an io::Result
    // the Result has a expect method if the Result wrappes an error 
    // then cause a panic! and it's cause the crash of the program
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}