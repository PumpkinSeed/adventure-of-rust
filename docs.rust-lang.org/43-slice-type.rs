fn main() {
    // slice does NOT have ownership

    // -------------
    // String Slices

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[..2]; // start with 0
    let slice = &s[3..]; // end with s.len()

    // ---------------
    // borrowing issue
    let mut s11 = String::from("hello world");

    let word = first_word(&s11); // immutable borrow occurs here

    s11.clear(); // error! mutable borrow occurs here

    println!("the first word is: {}", word); // immutable borrow later used here

    // --------------------------
    // String Literals Are Slices
    let s = "hello, world!";
    // s is a &str, it's a slice pointing to that specific point of the binary,
    // this is also why string literals are immutable, &str is an immutable reference
}

// fn first_word(s: &str) -> &str { ...
// here we can pass the string slice/literal directly, or String as slice &String[...]

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // iterator of the byte array, enumerate returns a tuple
        // there is a pattern to destruct the tuple, second is the reference represent a single byte

        if item == b' ' { // b means it will be a byte literal
            return &s[..i]; // return string literal reference of the part of the s
        }
    }

    &s[..] // return string literal reference of the whole s
}