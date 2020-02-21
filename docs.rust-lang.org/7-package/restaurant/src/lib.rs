mod front_of_house;

use front_of_house::hosting;
use front_of_house::serving::{take_order, serve_order};

fn serve() {
    hosting::add_to_waitlist();
    take_order();
    serve_order()
}

#[cfg(test)]
mod tests {
    use crate::serve;

    #[test]
    fn it_works() {
        serve()
    }
}
