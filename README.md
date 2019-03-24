# Rust for Node.js Developers

![](https://img.shields.io/badge/license-MIT-green.svg)

This guide full of examples is intended for people learning Rust that are coming from Node.js, although the vice versa can work too. I made this because I'm learning Rust myself, hoping maybe it'll also help other people with similar background and interest. I tried to learn idiomatic rust along the way, but please feel free to send PR if you find something that can be improved.

## Contents

- [Examples](#examples)
  - [comments](#comments)
- [Contributing](#contributing)
- [Acknowledgement](#acknowledgement)
- [License](#license)

## Examples

All sample code is available in [examples/](examples/)

Use `cargo-script` for running Rust `.rs` codes:

```
# Install cargo-script
cargo install cargo-script

# Run filename.rs
cargo script filename.rs
```

Or you can use [Rust Playground](https://play.rust-lang.org/) too.

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
/// Documentation comments must come before what they document. The code will not compile if we place this comment inside the function!
fn main() {
    // This is a line comment
    // that extend multiple lines like this.

    /*
      Note that Rust supports block comment too, but line comments are preferred in general.
    */
}

````

## Contributing

Pull requests are welcome!

Please submit a pull request for new interesting additions or for general content fixes.

If updating code, update both the README and the code in the [examples](examples/) folder.

## Acknowledgement

This project is inspired by [Golang for Node.js Developers](https://github.com/miguelmota/golang-for-nodejs-developers) repo.

## License

[MIT](LICENSE)
