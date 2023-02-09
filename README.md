# `boolalg-rs` a boolean algebra library to practice language parsing

pretty empty for now, but it will end up being useful someday. 

mostly defines an `Expr` enumeration and an `evaluate` function. There's also the ability to define a context as a `HashMap` and substitute variables with their value.

To be potentially turned into a wasm project. 

### TODO

- [ ] make it easier to use and test => add an interface
- [ ] restructure the code into a more Model View Controller format => make it a library?
- [ ] i'm out of stuff but i always have three points here.

### Inspirations/Credits

This project was motivated by my hate of my CS-101 class. What a learning experience that was. 

The parsing is heavily inspired by this resource right here: [Parser In Rust](https://adriann.github.io/rust_parser.html). It's very detailed and only needed a little coaxing to work with my integration. 