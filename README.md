# What?

a recursive descent parser for calculating arithmetic expressions written in Rust

# Why?

just to get better at writing Rust code and also learning how to write a parser

# How to use?

you can just use `cargo run` like below to evaluate the expression:

```bash
cargo run -- eval '-(23.56e3 + 2e-3) - (-10.2) * (2 + 8.2 + 2 * (4 + 5)) ^ (-1.2 - 1)'
```

use the following if you want to get the abstract syntax tree:

```bash
cargo run -- ast '-(23.56e3 + 2e-3) - (-10.2) * (2 + 8.2 + 2 * (4 + 5)) ^ (-1.2 - 1)'
```
