fn main() {
    //println!("Hello, world!");

    let  mut s = String::from("Hello");//Requests memory it needs

    s.push_str(", world!"); //push_str() appends a literal to a string

    println!("{}", s); //Prints Hello, World!

    //In Rust, once a variable is out of scope, the drop function is invoked
    //automatically that frees the memory.

    //This implementation of automatically invoking drop at the end of the 
    //closing braces is available in C++ and is used to deallocate resources
    //at the end of an item's lifetime and is called (RAII) - Resource Acquisition
    // Is Initialization.

    //Move: Ways that variables and Data Interact
    let x = 5;
    let y = x; //This will run without errors
    //Bind the value of x to y

    println!("x : {} and y : {}", x, y);

    // Let's test this on a string literal.
    let s1 = String::from("Trolling port 5001");
    let s2 = s1;//Move does not implement the copy trait

    //Let's attempt accessing s1
    println!("{}",s2);// Can only work if s1 = s1.clone()

    //A string is made up of three parts: 
    // - a pointer to the memory that holds the contents of the string.
    // - a length - how much memory in bytes, the contents of the String are currently using
    // - a capacity - the total amount of memory the String has received from the operating 
    //system. 

    // Stack-Only data: copy

    // Types such as integers have a known size at compile time and so are entirely stored
    // on the stack, so copies of the actual values are actually easy to make.


    //Ownership and Functions
    let ss = String::from("Take care");

    take_ownership(ss);
    
    let x = 5;  // x comes into scope
    make_copy(x);

    //Returning values by functions can be used to transfer ownership.
    //Returning ownership of parameters.
    let sy = String::from("hello");

    let (sy1, len) = calculate_length(sy);

    println!("The length of '{}' is {}.", sy1, len)
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
} //'drop' called, memory freed

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}
//The possibility of returning multiple values
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); //len() returns the length of a string

    (s, length)
}//In this block, the issue is that we have to return the String to the 
// calling functions so that we can use the String after the call to 
//calculate_length.


/*
Here is how to define and use a calculate_length function that has a reference to an object
as a parameter instead of taking ownership of the value.
*/

/*
fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


} 
fn calculate_length(s: &String) -> usize {
    s.len()
}
*/
