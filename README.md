# tylar

**Ty**pe-**L**evel **A**rithmetic in **R**ust.

Supports addition, subtraction, multiplication and division of (small-ish*) type-level integers.

Inspired by Björn Buckwalter's [numtype](https://github.com/bjornbm/numtype) library for Haskell. Brought to Rust thanks to multidispatch and associated types.

<sup>&#42; Numbers in the range (–50,50) should mostly work, depending on the operations. Typechecking might be rather slow.</sup>

## Example
```rust
// N2 is the type for -2, P5 is +5, Out is the result type
// of the addition; new() creates a new instance (actually a
// no-op, since the types are zero-sized) and into() turns
// the object into an integer value, computed at compile-time
// due to static dispatch.
let result: i32 = <N2 as Add<_,P5>>::Out::new().into();
println!("-2 + 5 = {}", result); // prints `-2 + 5 = 3`
```
For more, see [examples/basics.rs](examples/basics.rs) and run `cargo run --example basics`.

Unfortunately these examples (and therefore also `cargo test`) do not work with Rust 1.0, but *tylar* itself is compatible and usable with Rust 1.0.
