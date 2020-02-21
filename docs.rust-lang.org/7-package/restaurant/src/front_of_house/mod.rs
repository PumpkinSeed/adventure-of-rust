pub mod hosting {
    pub fn add_to_waitlist() {println!("add to waitlist");}
    pub fn seat_at_table() {println!("seat at table");}
}

pub mod serving {
    pub fn take_order() {println!("take order");}
    pub fn serve_order() {println!("serve order");}
    fn take_payment() {println!("take payment");}
}
