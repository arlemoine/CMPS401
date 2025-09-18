/*
    This program demonstrates subprograms (functions) in Rust:
    - Simple function with no params/returns
    - Functions with parameters and return values
    - Borrowing & mutation using references (& and &mut)
    - Working with slices 
    - Recursion

    Pass by Reference:
    - Read-only reference: &myVar
    - Mutable reference: &mut myVar

    To compile and run this Rust file:
    $ rustc TSub.rs
    $ ./TSub
*/

fn main() {
    // 1) Simple function
    greet();

    // 2) "Borrowing" a string slice (&string) (Pass by reference)
    // Notice - this is an immutable (read only) reference and not a pointer like C/C++
    let string = String::from("Alice");
    greet_name(&string);

    // 3) Return values
    let s: i32 = add(3, 4);
    println!("add(3, 4) = {}", s);

    // 4) "Borrowing" with a mutable reference (&mut)
    let mut value = 10;
    println!("Before increment: {}", value);
    increment(&mut value);
    println!("After  increment: {}", value);

    // 5) Slices
    let nums = [1, 2, 3, 4, 5];
    let total = sum_slice(&nums[1..4]); // Pass a slice (sub array) of the array - indices 1, 2, 3
    println!("sum([2,3,4]) = {}", total);

    // 6) Recursion
    let n: u64 = 5;
    println!("factorial({}) = {}", n, factorial(n));

    // 7) Option return for fallible operations
    match divide(10, 2) {
        Some(q) => println!("10 / 2 = {}", q),
        None => println!("10 / 2 = undefined (division by zero)"),
    }
    match divide(10, 0) {
        Some(q) => println!("10 / 0 = {}", q),
        None => println!("10 / 0 = undefined (division by zero)"),
    }
}

// --- Subprogram definitions ---

// 1) No params, no return
fn greet() {
    println!("Greetings from greet() subprogram!");
}

// 2) Borrow a string slice parameter
fn greet_name(name: &str) {
    println!("Hello, {}!", name);
}

// 3) Parameters with a return value
fn add(a: i32, b: i32) -> i32 {
    a + b // expression (no semicolon) is the return value
}

// 4) Mutably borrow an i32 and modify it in place
fn increment(n: &mut i32) {
    *n += 1;
}

// 5) Take a slice and compute a sum
fn sum_slice(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// 6) Recursive factorial
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

// 7) Fallible division using Option (None for division by zero)
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

