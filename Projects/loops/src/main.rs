fn main() {
    //loop {
      //  println!("Again!");
    //}

    // Returning values from loops.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);


    // Conditional loops with while

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");

    //Looping through a collection with for

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value of [{}] is : {}", index, a[index]);
        index = index + 1;
    }
    
}
