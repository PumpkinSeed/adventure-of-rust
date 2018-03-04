// we defined the rand in the [dependencies]
// we can use it by calling the extern crate
extern crate rand;

// get the std::io it's attach a second prelude
// here we can use an umber of useful io-related things
use std::io;

// put the Ordering to the scope for the compare
use std::cmp::Ordering;

// we would use a method of the Rng which 
// requires be in scope to work
use rand::Rng;

// the main function is the entry point of the program
fn main() {
    // rand::thread_rng() get a copy of the random number generator
    // gen_range waits two arguments, min and max
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // the println!() is a macro that prints a string to the screen
    println!("Guess the number!");

    // loop indicates an infinite loop, breaks in case of win
    loop {
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

        // here we 'shadow' the guess variable, it's lets us re-use the variable
        // Err(_) ignores the error just continue to the next iteration
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // cmp method can be called on anything that can be compared
        // takes a reference what we want to compare to
        // we use match statement to determine exactly what kind of 
        // Ordering it is, Ordering is an enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}