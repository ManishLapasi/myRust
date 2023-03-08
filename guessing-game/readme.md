# What I've learned

- I've commented parts of the program for easier understanding directly
- `read_line` returns a `Result` value, which is an `enum` that can be either `Ok` or `Err`
- `Result` instances have an `expect` method
- If `Result = Err`, then the program crashes and `expect` prints whatever you've specified. If `Result = Ok`, then `expect` just returns the value to the specified variable.
- If there is no `expect`, program compiles with a warning
- If the variable is unused, the program compiles with a warning

- Add new package dependencies to `cargo.toml`. The next build will download and install the newly added packages
- `cargo.lock` similar to `packages.lock` in js
- `cargo update` will update packages in `cargo.toml` to their latest versions

- `cargo doc --open` reads the `cargo.toml` file and downloads and opens up documentation, super useful imo

- `read_line` takes in input as a string. To convert to a suitable type, you have to `trim()` then `parse()` into a suitable type
- `loop` starts a loop. There are also `break` and `continue` statements