use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` alse has a type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behaviour of this method
        // depends on the varian of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: Any empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printin to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(4);
    list = list.prepend(8);
    list = list.prepend(6);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
