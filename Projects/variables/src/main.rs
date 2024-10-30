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

/*
 * let x = 9;
 * let x = x + 19;
 * let x = x ** 2;
 * println!("The value of x is: {}", x);
 */


 /*
  * if statements, should return a boolean value by default
  * if and else if statements, used in the same way as in other programming languages
  * let number =  4;
  if number % 4 == 0 {
  --} else if number % 2 == 0 {
  --} else {
  }
  */


 /*
  * Types (scalar types, compund typess)
  * scalar types(Integers, floating point numbers,
  * booleans, characters(unicode, ))
  * compound types (tuples,arrays)  
  */
    //tuples
    //- Have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, z) = tup;

    println!("The value of y & z is: {} {}", tup.1,  z);

    //accessing a value from a tuple using period.
    let  x: (i32, f64, u8) = (500, 6.4, 8);

    //let five_hundred = x.0;

    //let six_point_four = x.1;

    //let _eight = x.2;

    println!("Index 1 is :{}", x.1);

    /*
     * Array Type
     * - Have a fixed length.
     * - Collection of multiple elements of the same type.
     * - The elements going into an array are comma separated in square brackets.
     */

    /*
    Using if in a let statement
    let condtion = true;
    let number = if condition {
    5
    } else {
     6
     };
     println!("The value of number is: {}", number);
     }
     */
}

