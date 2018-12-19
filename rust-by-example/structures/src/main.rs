#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

struct Pair(i32, f32);

// A tuple struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Shorthand field init
    let name = "Peter";
    let age = 23;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Struct update syntax
    let new_point = Point { x: 0.1, ..point };
    println!("new_point coordinates: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instaniate a unit struct
    let _nil = Nil;

    // Instaniate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle {
        p1: Point { x: 1.1, y: 2.5 },
        p2: Point { x: 4.2, y: 6.7 },
    };

    rect_area(rect);

    let new_rect = square(Point { x: 1.2, y: 2.1 }, 5.5);

    println!("{:?}", new_rect);
}

fn rect_area(rect: Rectangle) {
    let Rectangle {
        p1: point1,
        p2: point2,
    } = rect;

    let x_diff = point1.x - point2.x;
    let y_diff = point1.y - point2.y;

    println!("Area is: {}", x_diff * y_diff);
}

fn square(p: Point, wh: f32) -> Rectangle {
    let Point { x, y } = p;
    Rectangle {
        p1: p,
        p2: Point {
            x: x + wh,
            y: y + wh,
        },
    }
}
