fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;

    // Mutability
    // ----------

    let _immutable_binding = 1;
    let mut mutable_binding = 0;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    for i in 1..42000 {
        mutable_binding += i;
    }

    println!("After mutation: {}", mutable_binding);

    // Scope and Shadowing
    // -------------------

    // This binding lives in the main function
    let long_lived_bindig = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // This binding *shadows* the outer one
        let long_lived_bindig = 5_f32;

        println!("inner long: {}", long_lived_bindig);
    }
    // short_lived_binding doesn't exist in this scope

    println!("outer long: {}", long_lived_bindig);

    let long_lived_bindig = 'a';

    println!("outer long: {}", long_lived_bindig);

    // Declare first
    // -------------

    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Err! Use of uninitialized binding
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
