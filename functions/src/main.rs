fn main() {
    print_number(12)
}

fn print_number(x: i32) {
    println!("x is {}", x);
    let mut x = 12;
    x = add_one(x)
}

fn add_one(x: i32) -> i32 {
    x+1
}