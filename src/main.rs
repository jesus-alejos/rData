use std::io;
fn main() {
    operations();
}
fn operations() {
    let num_1: f32 = 1.111111111;
    let num_2: f32 = 0.33333333333333333333333333333;
    println!("Sum: {}", num_1 + num_2);
    let num_3: u32 = 17;
    let num_4: u32 = 68;
    println!("{num_3} + {num_4} = {}", num_3 + num_4);
    println!("{num_3} - {num_4} = {}", num_3 * num_4);
    // println!("{num_3} * {num_4} = {}", num_3 * num_4);
    //    println!("{num_3} / {num_4} = {}", num_3 / num_4);
    //   println!("{num_3} % {num_4} = {}", num_3 % num_4);
}
fn numbers() {
    // Unsigned integer: u8, u16, u32, u64, u128, usize
    // Signed integer: i8, i16, i32, i64, i128, isize
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}
fn booleans() {
    let is_true = true;
    print!("{}", is_true);
}
fn strings() {
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
