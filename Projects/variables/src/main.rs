fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

//constants vs variables
//- Constants are immutable
//- Variables are only immutable by default

//Shadowing
//- The let keyword is used to assign a new value to a variable instead of using mut
//- Allows for type modification unlike in mut where type can't be altered
//- Allows transformations on a variable while maintaining their immutability.

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
  * compound types (tuples,arrays)  
  */
    //tuples
    //- Have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", tup.1);

    //accessing a value from a tuple using period.
    let  x: (i32, f64, u8) = (500, 6.4, 8);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let eight = x.2;


    /**
     * Array Type
     * - Have a fixed length.
     * - Collection of multiple elements of the same type.
     * - The elements going into an array are comma separated in square brackets.
     */
}

