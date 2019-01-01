#![feature(core_intrinsics)]

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    print_type_of(&my_str);
    print_type_of(&my_string);

    let num = Number::from(30);
    println!("My number is {:?}", num);
    print_type_of(&num);

    // If From implemented, you get the Into for free.
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
    print_type_of(&num)
}
