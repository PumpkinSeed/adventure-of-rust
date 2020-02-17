fn main() {
    // ----------
    // References
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // passing s1 as reference, &s1

    println!("The length of '{}' is {}.", s1, len);

    // references, they allow yo refer to some value without taking ownership
    // the opposite of referencing is dereferencing, accomplished with the dereference operator, *.

    // reference refers but not own it, so the value it points to will not be dropped
    // having references as function parameters, we call it BORROWING
    // just as variables are immutable by default, so are refrences, we are not allowed to modify
    // something we have a reference to

    // ------------------
    // Mutable References

    let mut s12 = String::from("hello"); // s12 should be mutable
    change(&mut s); // should be passed as mutable reference

    // RESTRICTION: mutable variables can have only one mutable reference in a particular scope
    //    let mut s = String::from("hello");
    //    let r1 = &mut s;
    //    let r2 = &mut s; // cannot borrow 's' as a mutable more then once at a time
    // the benefit, Rust can prevent DATA RACES at compile time
    // data races cause undefined behaviour and can be difficult to diagnose

    // there can be curly brackets to create a new scope
    // allowing for multiple mutable references

    let mut s14 = String::from("hello");
    {
        let r1 = &mut s14;
    } // r1 goes out of scope
    let r2 = &mut s14;

    // --------
    // similar rule for combining mutable and immutable references
    let r1 = &s14; // no problem
    let r2 = &s14; // no problem
    let r3 = &mut s14; // BIG PROBLEM
    println!("{}, {}, and {}", r1, r2, r3);
    // cannot borrow `s14` as mutable because it is also borrowed as immutable

    // --------
    let r1 = &s14; // no problem
    let r2 = &s14; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s14; // no problem
    println!("{}", r3);

    // -------------------
    // Dangling References

    // dangling pointer: a pointer that references a location in memory that may have been given to someone else
    // Rust compiler guarantees that references will never be dangling references

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize { // accept String as reference, &String
    s.len()
} // s goes out of scope, it does not have ownership of what it refers to, nothing happens

fn change(s: &mut String) { // accept a mutable reference
    s.push_str(", world");
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    return &s // return a reference to the String, s
} // s goes out of scope, and is dropped, it's memory goes away, ERROR: missing lifetime specifier