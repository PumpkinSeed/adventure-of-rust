#![allow(unused_variables)]
fn main() {
    char_ex()
}

// Example of boolean primitive type
fn bool_ex() {
    let x = true;

    let y: bool = false;
    println!("{}", y);
}

fn char_ex() {
    // char in Rust represents a single unicode scalar value
    let x = 'x';
    let two_hearts = '💕';

    // char isn't a single byte, but four
    println!("{}", two_hearts)
}

fn numeric_ex() {
    let x = 42; // `x` has type `i32`.

    let y = 1.0; // `y` has type `f64`.

    // i8 -> fixed-sized integer
    // i16 -> fixed-sized integer
    // i32 -> fixed-sized integer
    // i64 -> fixed-sized integer
    // u8 -> fixed-sized unsigned integer
    // u16 -> fixed-sized unsigned integer
    // u32 -> fixed-sized unsigned integer
    // u64 -> fixed-sized unsigned integer
    // isize -> variable-sized integer
    // usize -> variable-sized unsigned integer
    // f32 -> fixed-sized float
    // f64 -> fixed-sized float
}

fn arrays_ex() {
    
}