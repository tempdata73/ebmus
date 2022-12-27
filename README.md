# EBMUS
**Simple proof-of-work library**. Given an arbitrary string, it finds a nonce such that, when
appended to the string, its BLAKE 3 hash starts with a given number of hexadecimal zeros.

# How to use
By default:
```rust
use ebmus::Puzzle;

let difficulty = 5;
let template = "EBMUS:::"
let mut puzzle = Puzzle::new(template);

puzzle.solve(difficulty).unwrap();
println!("{}", puzzle.to_string()); // EBMUS:::1437302
println!("{}", puzzle.nonce); // 1437302
println!("{}", puzzle.hash()); // 000004c14153250a5234dfdca6a4a40c09ce8545fbe323463567631d05759516
```
If you are using the "template" feature:
```rust
use ebmus::template::Template;

let template = Template::new("EBMUS"); // EBMUS:  1672169715:9doEQzRH3U76KKH2:::
let mut puzzle = Puzzle::new(template);

//...
```

You can create your own custom template, as long as it implements the `ToString` trait. For example,
the "template" feature contains a `Template` struct defined by

```rust
struct Template {
  name: String,
  timestamp: i64,
  token: String,
}
```
resolves to the string "EBMUS:  1672169715:9doEQzRH3U76KKH2:::".

# TODO
- add documentation
- add tests
- instead of looking for hexadecimal zeros, find binary zeros.
