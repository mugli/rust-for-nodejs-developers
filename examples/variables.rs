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