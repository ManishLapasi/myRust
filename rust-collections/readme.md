# What I've learned

- pass vectors by reference
- pass vectors through mutable references if you're making changes
- cannot index outside vector's range, throws error
- can index outside range if you use `get`, this will return "None". Can be used for error handling without crashing
- can store multiple types in a single vector by using enums