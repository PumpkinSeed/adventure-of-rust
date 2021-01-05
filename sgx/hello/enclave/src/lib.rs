#![crate_name "helloclave"]
#![crate_type "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {
    let str_slice = unsafe {slice::from_raw_parts(some_string, some_len)};
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is an in-Enclave ";

    // An array
    let word:[u8;4] = [82, 117, 115, 116];

    // An vector
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8").as_str();

    // OCALL for normal word for output
    println!("{}", &hello_string);

    sgx_status_t::SGX_SUCCESS
}
