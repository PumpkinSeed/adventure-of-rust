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
}