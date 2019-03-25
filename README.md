# Rust for Node.js Developers

![](https://img.shields.io/badge/license-MIT-green.svg)

(**Work in progress**. Done: 4 of 60 planned examples)

This guide full of examples is intended for people learning Rust that are coming from Node.js, although the vice versa can work too. I made this because I'm learning Rust myself, hoping maybe it'll also help other people with similar background and interest. I tried to learn idiomatic rust along the way, but please feel free to send PR if you find something that can be improved.

This is a [cookbook style](https://rust-lang-nursery.github.io/rust-cookbook/) guide. However, it doesn't explain how to install Rust, what cargo is etc. If you need help with that, this might be a good starting point, which is written in tutorial style: https://github.com/Mercateo/rust-for-node-developers

## Contents

- [Examples](#examples)
  - [comments](#comments)
  - [printing](#printing)
  - [variables](#variables)
  - [types](#types)
    - [bool](#types)
    - [number](#types)
    - [string](#types)
    - [array](#types)
    - [object](#types)
    - [function](#types)
    - [closure](#types)
- [Contributing](#contributing)
- [Acknowledgement](#acknowledgement)
- [License](#license)

## Examples

All sample code is available in [examples/](examples/)

Use `cargo-script` for running Rust `.rs` codes:

```bash
# Install cargo-script
cargo install cargo-script

# Run filename.rs
cargo script filename.rs
```

_Note: Sometimes the Rust source will have comment like this:_

```rust
// cargo-deps: time="0.1.25"
```

_This is only to let `cargo script` know that it needs to download `version 0.1.25` of [`time` crate](https://crates.io/crates/time) before running the file. You don't need that comment in a Rust project file created with `cargo new` command._

### comments

---

#### Node.js

```js
// This is a line comment

/*
  This is a block comment
*/
```

#### Rust

````rust

/// Documentation comments look like this and support markdown notation.
/// # Examples
///
/// ```
/// let five = 5
/// ```
///
/// Documentation comments must come before what they document.
/// The code will not compile if we place this comment inside the function!
fn main() {
    // This is a line comment
    // that extend multiple lines like this.

    /*
      Note that Rust supports block comment too, but line comments are preferred in general.
    */
}

````

### printing

---

#### Node.js

```js
console.log('hello world');
console.log('hello %s', 'world');
console.log('hello %d %s', 5, 'worlds');
```

Output

```bash
hello world
hello world
hello 5 worlds
```

#### Rust

`println!` is a [macro](https://doc.rust-lang.org/book/ch19-06-macros.html)

```rust

fn main() {
    println!("hello world");
    println!("hello {}", "world");
    println!("hello {} {}", 5, "worlds");
}

```

Output

```bash
hello world
hello world
hello 5 worlds
```

### variables

---

#### Node.js

```js
// function scoped
var foo = 'foo';

// block scoped
let bar = 'bar';

// constant
const qux = 'qux';
```

#### Rust

Variables are block scoped in Rust. They are immutable unless you declare them with `mut` keyword. But they can be shadowed!

```rust
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // All variables are block scoped.

    // Variables are by default immutable.
    let foo: &str = "foo";

    // And you can redeclare variables! This is called shadowing.
    // Useful during data transformation when you don't want to
    // name a lot of intermediate and temporary variables.
    let foo: isize = 42;

    // Type inferred
    let bar = "bar";

    // Mutable
    let mut baz: &str = "baz";
    baz = "bazbaz";

    // Rust has constants. But they are different than immutable variables.
    // Constants evaluate at compile-time, where variables evaluate at run-time.
    // Constant values are effectively an alias for a literal value.
    // In short, constants represent a value, not a memory address.
    const MEANING: isize = 42;
}

```

### types

---

#### Node.js

```js
// primitives
const myBool = true;
const myNumber = 10;
const myString = 'foo';
const mySymbol = Symbol('bar');
const myNull = null;
const myUndefined = undefined;

// object types
const myObject = {};
const myArray = [];
const myFunction = function() {};
const myError = new Error('error');
const myDate = new Date();
const myRegex = /a/;
const myMap = new Map();
const mySet = new Set();
const myPromise = Promise.resolve();
const myGenerator = function*() {};
const myClass = class {};

function makeAdder(x) {
  // JavaScript closure
  return function(y) {
    return x + y;
  };
}

var add5 = makeAdder(5);
var add10 = makeAdder(10);

console.log(add5(2)); // 7
console.log(add10(2)); // 12
```

#### Rust

```rust
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

```

## Contributing

Pull requests are welcome!

Please submit a pull request for new interesting additions or for general content fixes.

If updating code, update both the README and the code in the [examples](examples/) folder.

## Acknowledgement

This project is inspired by [Golang for Node.js Developers](https://github.com/miguelmota/golang-for-nodejs-developers) repo.

## License

[MIT](LICENSE)
