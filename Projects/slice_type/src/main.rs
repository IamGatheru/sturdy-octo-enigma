//A slice lets you reference a contiguous sequence of elements in a collection rathet tha the 
//entire collection.

//TASK
/*
Write a function that takes a string and returns the first word it finds in that string.
 */
fn first_word(s:&String) -> usize {
    //Here's an improvement of this first line
    // fn first_word(s: &str) -> &str { 
    // fn first_word(s: &String) -> &str{
    //But then you will have to use slices e.g [..], [..i], [i..j] etc
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
fn main() {
    let mut s = String::from("Hello World!");

    let word = first_word(&s); //word will get the value of 5
    s.clear(); //Empties the string making it equal to ""
    println!("The first word is {}", word);

    // word still has the value 5 here, there's no more string that
    //we could meaningfully use the value 5 with.
    //Word is now totally invalid!

}
