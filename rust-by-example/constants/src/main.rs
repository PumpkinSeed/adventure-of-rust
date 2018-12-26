// Globals are declared outside all other scopes.
static LANG: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANG);
    println!("The thresjold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Err! Cannout modify a `const`.
    // THRESHOLD = 5;
}
