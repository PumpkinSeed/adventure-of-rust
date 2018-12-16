fn main() {
    /** 
     * 
     * Formatted print
     * 
    */
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    // Special formating
    println!("{} of {:b} people know binary, the orher half doesn't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    let width = 6;
    println!("{number:>0width$}", number=1, width=width);

    #[allow(dead_code)]
    struct Strucutre(i32);
    // println!("This struct `{}` won't print...", Strucutre(3));
    // `main::Strucutre` cannot be formatted with the default formatter 

    let pi = 3.141592;
    println!("Pi is roughly {:.*}", 3, pi);

    /**
     * 
     * Debug 
     * 
     */

    // This struct cannot be printed either with `fmt::Display` or with `fmt::Debug`
    struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation 
    // required to make this `struct` printable with `fmt::Debug`
    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person{name,age};
    println!("{:#?}", peter);
}