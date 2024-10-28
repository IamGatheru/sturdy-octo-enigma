fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

//constants vs variables
- Constants are immutable
- Variables are only immutable by default

//Shadowing
- The let keyword is used to assign a new value to a variable instead of using mut
- Allows for type modification unlike in mut where type can't be altered
- Allows transformations on a variable while maintaining their immutability.

/**
 * let x = 9;
 * let x = x + 19;
 * let x = x ** 2;
 * println!("The value of x is: {}", x);
 */


 /**
  * Types (scalar types, compund typess)
  * scalar types(Integers, floating point numbers,
  * booleans, characters(unicode, ))
  * compound types (tuples,arraySS )  
  */
}
