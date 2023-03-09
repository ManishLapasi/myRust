# What I've learned

- Ownership in rust
- assigning a string value to another string value is equivalent to shallow copy
- if you want to deep copy, you can use an operation like `clone()`

- you can also return values from a function!
- rust does gc by 'dropping' values once they go out of scope, that's why it's super fast and secure
- you can't drop the same value twice, hence the default behaviour or making shallow copies and 'moving' values 

- references are immutable by default

- so how exactly does rust prevent data race situations? by preventing to mutable references to one value at the same time!
- look at `main.rs` for an example of this!
- you can have multiple *immutable* references, since in that case you're just reading the data
- you cannot have both a mutable and an immutable reference for one variable at the same time! that's because when you're reading something, you wouldn't like someone else to suddenly change the value from out under you!
- references must be valid at all times
- you can have any number of immutable references, or you can have 1 mutable reference and no immutable references at the same time 
- "at the same time" here means "in the same scope"