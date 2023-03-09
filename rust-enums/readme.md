# What I've learned

- `enum` is a useful way to define values that can be of different datatypes
- `Option` enum is Rust's way of trying to implement `Nulls`. An `Option` can be a `Some(T)` or `None`, where `T` is any datatype
- We cannot operate `T` with `Option(T)`, which means we are forces to make sure we are not operating on `Nulls`

- We can use `match` to match unspecified values to particular control flow as well!
- We can perform operations on `Some(T)` to change it to `Some(T')` via match control!
- `if let` syntax is a more concise way to evaluate based on matches!