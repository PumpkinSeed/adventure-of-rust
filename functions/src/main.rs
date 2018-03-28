// the main function is the 
// entry point of the rust program
fn main() {
    print_number(12)
}

// waits to variable both int32
// prints the sum of them
fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn print_number(x: i32) {
    println!("x is {}", x);
    let mut x = 12;
    x = add_one(x)
}

// Rust functions return exactly one value, 
// and you declare the type after an ‘arrow’
fn add_one(x: i32) -> i32 {
    x+1
}

// Rust is primarily an expression-based language. 
// There are only two kinds of statements, 
// and everything else is an expression.
// So what's the difference? Expressions return a value, 
// and statements do not.

// implements an early return
fn add_one(x: i32) -> i32 {
    return x

    x+1
}

// Rust has some special syntax for ‘diverging functions’, 
// which are functions that do not return
fn diverges() -> ! {
    panic!("This function never returns!");
}

// We can also create variable bindings which point to functions
// f is a variable binding which points to a function that takes 
// an i32 as an argument and returns an i32. For example

// Without type inference:
let f: fn(i32) -> i32 = plus_one;

// With type inference:
let f = plus_one;