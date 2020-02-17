fn main() {
    // string literal has known size at compile time and it's immutable
    // so it's fast and efficient because compiler can keep on the stack

    // String is mutable:
    // 1. Memory must be requested from the OS at compile time
    // 2. Need a way of returning this memory to the operating system when we are done

    // Second part is tricky, mostly done by GC or with exactly one allocate with exactly one free
    // Rust way: memory automatically returned once the variable that owns it goes out of scope
    let mut s = String::from("hello");

    // drop(s); -> special function, it means return the memory
    // Rust calls drop automatically at the closing curly bracket
    s.push_str(", world!");

    println!("{}", s);

    // --------------------------------------
    // Ways Variables and Data Interact: Move
    // double free error: when two variable pointer points to the same heap allocation
    // freeing memory twice lead to memory corruption, lead to security vulnerabilities

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // s1 is invalid

    // Instead trying to copy the allocated memory Rust considers s1 to no longer be valid
    // and DOESN'T NEED TO FREE anything when s1 goes out of scope

    // shallow copy - duplicate as little as possible
    // deep copy  - duplicates everything

    // so that sounds like a shallow copy, but because Rust invalidates the first variable,
    // it's known as a MOVE, so s1 was moved into s2

    // Rust will never automatically create "deep" copies of data

    // ---------------------------------------
    // Ways Variables and Data Interact: Clone

    // if we want to deeply copy the heap data, not just the stack data, we can use a common method
    // called clone


    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // ---------------------
    // Stack-Only Data: Copy

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // types such as integers that have known size at compile time are stored on stack
    // so copies of the actual values are quick to make

    // in that case x won't be invalid after we create y

    // -----------------------
    // Ownership and Functions

    // semantics for passing a value to a function are similar to those for assigning a value to a variable
    // move or copy

    let so = String::from("test"); // s come into scope
    takes_ownership(so); // s's value moves and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x moves bit i32 has Copy trait so it's okay to still use x

    // -----------------------
    // Return Values and Scope

    // returning values also can transform the ownership

    let s12 = String::from("hello");     // s12 comes into scope

    let s13 = takes_and_gives_back(s12);  // s12 is moved into takes_and_gives_back, which also moves its return value into s13
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called, memory freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing special happens


// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}