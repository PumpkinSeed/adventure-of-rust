use std::mem;

type NanoSecond = u64;
type Inch = u64;

// Attribute to silence warning.
#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    /**
     * Literals
     */
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variables in bytes
    println!("size of `x` in bytes (u8): {}", mem::size_of_val(&x));
    println!("size of `y` in bytes (u32): {}", mem::size_of_val(&y));
    println!("size of `z` in bytes (f32): {}", mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", mem::size_of_val(&f));

    /**
     * Inference
     */
    // Because of the annotation, the compiler knowns that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array)
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`)

    // Insert `elem` in the vector.
    vec.push(elem);
    // Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

    println!("{:?}", vec);

    /**
     * Aliasing
     */
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Type aliases DON'T provide any extra safety, because
    // aliases are NOT new types
    println!(
        "{} nanoseconds + {} inches = {} uint?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
