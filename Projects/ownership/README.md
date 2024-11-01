Rules of ownership.
- Each value in Rust has a variable that's called its owner.
- There can be only one owner at a time.
- When the owner goes out of scope, the value will be dropped.


//Shallow and Deep copying
Traits: clone, copy
Types that are copy:
- All the integer types
- The Boolean type, bool, with value true and false
- The character type, char
- Tuples, if they contain types that are copy
