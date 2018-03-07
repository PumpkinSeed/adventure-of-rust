fn main() {
    // variable binding
    let x = 5;
    println!("{}", x);

    // left-hand side of a let statment is 'patterns'
    let (x, y) = (1, 2);
    println!("{} + {} = {}", x, y, x+y);

    // type annotations
    let x: i32 = 12;
    println!("{}", x);

    // variable bindings are immutable by default
    // let imm = 5;
    // imm = 23;
    // this code won't compile
    // mut binding variable as mutable
    let mut mutable = 22;
    println!("{}", mutable);
    mutable = 42;
    // Rust will interpret this as a request to interpolate some sort of value
    // moustaches -> string interpolation is a computer science term 
    // that means 'stick in the middle of a string'
    println!("{}", mutable);

    // variable bidings can be shadowed, this means that a later 
    // variable bindig with the same name as another bindig that is currently
    // in a scope will override the previous binding 
    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8".
        let x = 12;
        println!("{}", x); // Prints "12".
    }
    println!("{}", x); // Prints "8".
    let x =  42;
    println!("{}", x); // Prints "42".

    // shadowing enables us to rebind a name to a value of a different type
    // also possible to change the mutabillity of a bindig
    let mut x: i32 = 1;
    x = 7;
    let x = x; // `x` is now immutable and is bound to `7`.

    let y = 4;
    let y = "I can also be bound to text!"; // `y` is now of a different type.
}
