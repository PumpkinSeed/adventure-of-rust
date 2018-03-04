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
    println!("{}", mutable);

}
