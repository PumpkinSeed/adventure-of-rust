const MAX_POINTS: u32 = 100_000;

fn main() {
    // Constants
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // Mutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Variable shadowing into the same name
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);
    // By using let, we can perform a few transformations on a value 
    // but have the variable be immutable after those transformations 
    // have been completed.
    // We can change the type of the value but reuse the same name.
}