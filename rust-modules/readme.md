# What I've learned

- making `enum` public makes all its elements public
- making `struct` public doesn't make its elements public, we have to specify for each element with the `pub` keyword

- normal practice is to reference the full path when calling entities belonging to a library, with the `use` keyword

- you can store multiple struct definitions in multiple files and reference them using `mod` and `use`
- make sure to have the `src/lib.rs` file

- peep the two different ways to call modules:
    - defined in the same file `src/back_of_house.rs`
    - defined in child files `src/front_of_house.rs`, in which case the modules are located in the `src/front_of_house/` directory