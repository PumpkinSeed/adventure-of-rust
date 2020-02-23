use std::fs::File;
use std::io::Read;
use std::io;
use std::io::ErrorKind;

fn main() {
    // Rust's commitment to reliability extends to error handling
    // Groups error into two major categories: recoverable and unrecoverable errors
    // recoverable: file not found
    // unrecoverable: location on the end of the array not exists

    // Result<T, E> enum for recoverable
    // panic! for unrecoverable

    // --------------------
    // Unrecoverable errors

    // Unwinding the Stack or Aborting in Response to a Panic
    // into Cargo.toml
    // [profile.release]
    // panic = 'abort'

    panic!("crash and burn");

    // panic comes from the library because of a bug
    let v = vec![1, 2, 3];
    v[99];
    // in C it's buffer overread, returns that element in the vector, even though the memory doesn’t belong to the vector
    // panic points to libcore/slice/mod.rs, implementation of slice

    // RUST_BACKTRACE env var will allow Rust to print the backtrace

    // ------------------------------
    // Recoverable Errors with Result

    // enum Result<T, E> {
    //     Ok(T), 
    //     Err(E),
    // }
    // Result Enum, where T and E are generic type parameters

    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // NOTE: Option enum and Result enum have been brought into by prelude
    // so we don't need to specify Result::

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem opening the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // reduce the amount of matches, using the Result's unwrap and closure

    // -----------------------------------------------
    // Shortcuts for Panic on Error: unwrap and expect

    // Result has many helper methods
    // unwrap -> returns the Ok part and if it's Error calls panic
    let f = File::open("hello.txt").unwrap();

    // Another method is expect, similar to unwrap, let us choose the panic! message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // ------------------
    // Propagating Errors

    // When you’re writing a function whose implementation calls something that might fail,
    // instead of handling the error within this function, you can return the error
    // to the calling code so that it can decide what to do
    let s = match read_username_from_file() {
        Ok(s) => s,
        Err(e) => panic!(e)
    };

    // -------------------------------------------------
    // A Shortcut for Propagating Errors: the ? Operator
    let s = match read_user_from_file() {
        Ok(s) => s,
        Err(e) => panic!(e)
    };
    // the ? placed after a Result value
    // if Ok, the value of the Ok returns
    // if Err, it will returned from the whole function
    // ? calls the from function on the Err defined in the From trait
    // it will convert the error from the received type to the return type of the current function

    let s = match read_from_file() {
        Ok(s) => s,
        Err(e) => panic!(e)
    };

    let s = match read_file() {
        Ok(s) => s,
        Err(e) => panic!(e)
    };

    // ----------------------------------------------------------
    // The ? Operator Can Be Used in Functions That Return Result

    // ? only allowed to use in a function that returns Result or Option
    // or another type that implements std::ops::Try
    // main() is special one valid return type is (),
    // another Result<T, E>
    //    fn main() -> Result<(), Box<dyn Error>> {
    //        let f = File::open("hello.txt")?;
    //
    //        Ok(())
    //    }
    // Box<dyn Error> is called a trait object, currently means any kind of error
    // using ? in a main function with this return type is allowed
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_user_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

