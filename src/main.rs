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
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
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
    let t = true;

    let f: bool = false; // with explicit type annotation
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
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

// Compound Types
// Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

fn compound_types() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    // We can also access a tuple element directly by using a period (.)
    // followed by the index of the value we want to access. For example:

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // array
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    // Invalid Array Element Access
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    // if we place a semicolon at the end of the line containing x + 1,
    // changing it from an expression to a statement, we’ll get an error:
    x + 1
}
fn functions() {
    // Statements and Expressions
    // Function bodies are made up of a series of statements optionally ending in an expression. So far, the functions we’ve covered haven’t included an ending expression, but you have seen an expression as part of a statement. Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value.

    // Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust.
    // Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11.
    // Expressions can be part of statements: in Listing 3-1, the 6 in the statement let y = 6;
    //  is an expression that evaluates to the value 6.

    //Function definitions are also statements; the entire preceding example is a statement in itself.
    // Statements do not return values. Therefore, you can’t assign a let statement to another variable,

    //Calling a function is an expression. Calling a macro is an expression.
    //A new scope block created with curly brackets is an expression, for example:
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Functions can return values to the code that calls them.
    // We don’t name return values, but we must declare their type after an arrow (->).
    //  In Rust, the return value of the function is synonymous with the value of the final
    // expression in the block of the body of a function.
    // You can return early from a function by using the return keyword and specifying a value,
    // but most functions return the last expression implicitly
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}
fn controlFlow() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Remember that blocks of code evaluate to the last expression in them,
    // and numbers by themselves are also expressions.
    // In this case, the value of the whole if expression depends on which block of code executes.
    // This means the values that have the potential to be results from each arm of the if must be the same type;
    // in Listing 3-2, the results of both the if arm and the else arm were i32 integers.
    // If the types are mismatched, as in the following example, we’ll get an error:
    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    // If you have loops within loops, break and continue apply to the innermost loop at that point.
    // You can optionally specify a loop label on a loop that you can then use with break or continue
    // to specify that those keywords apply to the labeled loop instead of the innermost loop.
    // Loop labels must begin with a single quote

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
fn memory() {
    let x = 5;
    let y = x; // copy

    let s1 = String::from("hello");
    let s2 = s1; // Move, not shallow copy

    // To ensure memory safety, after the line let s2 = s1;,
    // Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope.
    // Check out what happens when you try to use s1 after s2 is created; it won’t work:
    println!("{}, world!", s1);

    // If we do want to deeply copy the heap data of the String,
    // not just the stack data, we can use a common method called clone.
    // We’ll discuss method syntax in Chapter 5, but because methods are a common feature in many programming languages,
    // you’ve probably seen them before.
    // Clone operation more expensive
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);
    // But this code seems to contradict what we just learned:
    // we don’t have a call to clone, but x is still valid and wasn’t moved into y.
    // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack,
    // so copies of the actual values are quick to make.
    // That means there’s no reason we would want to prevent x from being valid after we create the variable y.
    //  In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.

    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
    // Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error. To learn about how to add the Copy annotation to your type to implement the trait, see “Derivable Traits” in Appendix C.
    // So, what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.}
}
fn ownership() {
    // REFERENCES ARE INMUTABLE BY DEFAULT
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn ownership2() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
fn ownership3() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    // Mutable and inmutable variables

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM
                 // println!("{}, {}, and {}", r1, r2, r3);
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn danglingRefs() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

// The solution here is to return the String directly:
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
//The Rules of References
// Let’s recap what we’ve discussed about references:
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.