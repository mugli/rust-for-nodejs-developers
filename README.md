# Rust for Node.js Developers (work in progress 😊)

![](https://img.shields.io/badge/license-MIT-green.svg)

This guide full of examples is intended for people learning Rust that are coming from Node.js, although the vice versa can work too. I made this because I'm learning Rust myself, hoping maybe it'll also help other people with similar background and interest. I tried to learn idiomatic rust along the way, but please feel free to send PR if you find something that can be improved.

## Contents

- [Examples](#examples)
  - [comments](#comments)
  - [printing](#printing)
  - [variables](#variables)
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

## Contributing

Pull requests are welcome!

Please submit a pull request for new interesting additions or for general content fixes.

If updating code, update both the README and the code in the [examples](examples/) folder.

## Acknowledgement

This project is inspired by [Golang for Node.js Developers](https://github.com/miguelmota/golang-for-nodejs-developers) repo.

## License

[MIT](LICENSE)
