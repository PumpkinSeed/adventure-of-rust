fn main() {
    // Type annotated
    let logical: bool = true;
    println!("{}", logical);

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation
    println!("{} {}", a_float, an_integer);

    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`
    println!("{} {}", default_float, default_integer);

    // Type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    println!("{}", inferred_type);
    inferred_type = 4294967296i64;
    println!("{}", inferred_type);

    // Mutable variable's value can be changed
    let mut mutable = 12;
    println!("{}", mutable);
    mutable = 21;
    println!("{}", mutable);

    // Error! The type of a variable can't be changed
    // mutable = true;

    // Variables can be overwritten with shadowing
    let mutable = true;
    println!("{}", mutable);

    /**
     *
     * Literals and operators
     *
     */

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer substraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability
    println!("One million is written as {}", 1_000_000u32);
}
