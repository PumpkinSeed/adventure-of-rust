fn main() {
    // modules: organize code within a crate into groups
    // modules also controls privacy, public/private

    // by using modules, we can group related definitions together
    // src/lib.rs and src/main.rs are called crate root
    //    crate
    //    └── front_of_house
    //          ├── hosting
    //          │       ├── add_to_waitlist
    //          │       └── seat_at_table
    //          └── serving
    //                  ├── take_order
    //                  ├── serve_order
    //                  └── take_payment

    // use std::collections::HashMap;
    // use std::io::Result as IoResult; // adding new name

    // -------------------------------
    // Re-exporting names with pub use

    //    mod front_of_house {
    //        pub mod hosting {
    //            pub fn add_to_waitlist() {}
    //        }
    //    }
    //
    //    pub use crate::front_of_house::hosting;
    //
    //    pub fn eat_at_restaurant() {
    //        hosting::add_to_waitlist();
    //        hosting::add_to_waitlist();
    //        hosting::add_to_waitlist();
    //    }

    // external code also can call hosting - crates.io

    // -----------------
    // External packages
    // Cargo.toml
    // [dependencies]
    // rand = "0.5.5"

    // use rand::Rng;
    // fn main() {
    //     let secret_number = rand::thread_rng().gen_range(1, 101);
    // }

    // std is part of the Rust language

    // ------------------
    // Using nested paths

    // use std::io;
    // use std::cmp::Ordering;
    // -- convert to
    // use std::{cmp::Ordering, io};

    // use std::io;
    // use std::io::Write;
    // -- convert to
    // use std::io::{self, Write};

    // -------------
    // Glob operator

    // bring all public items defined
    // use std::collections::*;

}