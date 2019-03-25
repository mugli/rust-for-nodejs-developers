#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // Primitives

    // Scalar primitives: integers
    let my_bool: bool = true;
    let my_int: isize = 10; // pointer size
    let my_int8: i8 = 10;
    let my_int16: i16 = 10;
    let my_int32: i32 = 10;
    let my_int64: i64 = 10;
    let my_int128: i128 = 10;

    // Scalar primitives: unsigned integers
    let my_uint: usize = 10; // pointer size
    let my_uint8: u8 = 10;
    let my_uint16: u16 = 10;
    let my_uint32: u32 = 10;
    let my_uint64: u64 = 10;
    let my_uint128: u128 = 10;

    // Scalar primitives: floats
    let my_float32: f32 = 10.5;
    let my_float64: f64 = 10.5;

    // Scalar primitives: Unicode characters, 4 bytes each
    let my_char: char = '🐶'; // Noticed the single quotes? 👈

    // Scalar primitives: bool
    let my_bool: bool = false;

    // Scalar primitives: "unit type". The only possible value is an empty tuple: ()
    let my_unit: () = ();

    // ----------------------------------------------------------------------
    // ----------------------------------------------------------------------
    // ----------------------------------------------------------------------

    // Compound primitives: Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0; // Accessing tuple. Noticed the dot? 👈

    // Compound primitives: Array
    // Arrays in Rust are different from arrays in some other languages
    // because arrays in Rust have a fixed length, like tuples.
    //
    // Since an array has a fixed size, even if the array’s elements
    // are modified, it cannot grow or shrink.
    //
    // Unlike a tuple, every element of an array must have the same type.
    //
    // If you need the collection to grow or shrink, you'll need to use a vector
    // instead of an array.
    let a = [1, 2, 3, 4, 5];
    let first = a[0]; // Accessing array

    // ----------------------------------------------------------------------
    // ----------------------------------------------------------------------
    // ----------------------------------------------------------------------

    // Custom type: Struct

    // A unit struct
    struct Nil;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    struct Point {
        x: f32,
        y: f32,
    }

    // Instantiate a struct
    let point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // ----------------------------------------------------------------------
    // ----------------------------------------------------------------------
    // ----------------------------------------------------------------------

    // Custom type: Enum

    enum WebEvent {
        // An `enum` may either be `unit-like`:
        PageLoad,
        // like tuple structs:
        KeyPress(char),
        // or like structures:
        Click { x: i64, y: i64 },
    }

    let pressed = WebEvent::KeyPress('x');
    let load = WebEvent::PageLoad;
    let click = WebEvent::Click { x: 20, y: 80 };

    // ----------------------------------------------------------------------
    // ----------------------------------------------------------------------
    // ----------------------------------------------------------------------

    // Function & closures
    fn f() {
        println!("I have been called!")
    };
    let my_function_pointer = f;
    my_function_pointer();

    // Closures in Rust are anonymous functions you can save in a variable or
    // pass as arguments to other functions.
    // They are also called lambda expressions or lambdas,
    // they can capture their enclosing environment.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    println!("closure_annotated: {}", closure_annotated(10));
    println!("closure_inferred: {}", closure_inferred(10));

    // A lot to unpack here!
    // For detail explanation: https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html
    fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |y| x + y)
    }
    let add_5 = make_adder(5);
    let add_10 = make_adder(10);
    println!("{}", add_5(2)); // Output: 7
    println!("{}", add_10(2)); // Output: 12
}
