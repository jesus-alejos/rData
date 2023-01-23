use std::io;
fn main() {
    println!("Hello, world!"); // println! is from std::macros meaning is a call to the native C runtime std library
    let mut input = String::new(); // A UTF-8–encoded, growable string
    let un_mutable_string = "New String"; // String with double quote
    let char = 'a'; // Character with single quote
    println!("Str_1: {un_mutable_string}\nStr_2: {char}");
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("{input}");
        }
        Err(error) => println!("error: {error}"),
    }
}
