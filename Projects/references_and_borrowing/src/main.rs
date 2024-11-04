//Here's how you would define and use a calculate_length function that 
//has a reference to an object as a parameter instead of taking ownership of the value.
fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() //&(ampersand) are prefix references and they allow you to
    //refer to some value without taking ownership of it.
}
//Having references as function parameters is called borrowing.

//Can we modify something we are borrowing? Let's find out.
/*
let s = String::from("hello");

change(&s);
}
fn  change(some_string: &String) {//use &mut String to make it mutable
    some_string.push_str(", world");
}*/

//Mutable references:
/*
let mut s = String::from("hello");
change(&mut s);

//Mutable references have one big restriction;
//You can have only one mutable reference to a particular scope.
//This is goin' to fail:

let r1 = &mut s;
let r2 = &mut s;

println!("{} {}", r1, r2)//Err cannot borrow s as mutable more than once at a time.
*/
//The mutability restriction helps prevent data races at compile time
//A data race is similar to a race condition and happens when these 
//three behaviours occur:
/*
- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronise access to the data.
 */

 //Good news, we can always use curly braces to create a new scope, allowing
 //multiple mutable references , just not simultaneous ones:

 /*
 let mut s = String::from("hello");

 {
    let r1 = &mut s;
 }//r1 goes out of scope here, so we can make a new reference with no
 //problems
 let r2 = &mut s;
  */

  //The Rules of References.
  /*
  -At any given time, you can have either but not both of the following:
  1. one mutable or any number of immutable references.
  2. or any number of immutable references 
  - References must always be valid.
  */
