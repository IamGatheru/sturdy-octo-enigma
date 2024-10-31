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
}
