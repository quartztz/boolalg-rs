# `boolalg-rs` a boolean algebra library to practice language parsing

pretty empty for now, but it will end up being useful someday. 

mostly defines an `Expr` enumeration and an `evaluate` function. There's also the ability to define a context as a `HashMap` and substitute variables with their value.

to be potentially turned into a wasm project. 

### The interface

a veeeeeery crudely implemented shell is given in `main.rs` to work with the given definitions and see if they are properly defined. its functioning is described here. First, an example: 

```console
> $: a = p v q
added => a = (p v q)
> !: p -> T
=> setting p to T
> ?: (p v F) ^ p
parsed => ((p v F) ^ p)
evaluated => T
> ?: $a
using prev def => (p v q)
evaluated => T
```

On prompt `> `, multiple options are available: 
 - `!: { char } ->  { T | F | - }`: update the evaluation context giving variable `{ char }` the value `T` or `F` depending on input. 
 - `$: { char } = { expr }`: define a new macro giving identifier `{ char }` value `{ expr }`. This is confusing. I know. At least the evaluation is unambiguous.
 - `?: { expr | $iden }`: evaluate an expression, either typed out in as `{ expr }` or addressed using a previously defined identifier and a dollar sign. 
 - `close`: closes the shell
 - `status`: prints the variable hashmap
 - `help`: show help (not implemented yet)

### TODO

- [x] make it easier to use and test => add an interface
- [x] restructure the code into a more Model View Controller format => make it a library?
- [ ] figure out wasm

### Inspirations/Credits

This project was motivated by my hate of my CS-101 class. What a learning experience that was. 

The parsing is heavily inspired by this resource right here: [Parser In Rust](https://adriann.github.io/rust_parser.html). It's very detailed and only needed a little coaxing to work with my integration. 