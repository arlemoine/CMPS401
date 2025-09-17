/*
    This Rust program demonstrates various variable types, mutability, type inference, type casting, and basic operations.
    It covers signed and unsigned integers, floating-point numbers, booleans, characters, strings, tuples, and arrays.
    It also illustrates mutable vs immutable variables, type casting, and basic arithmetic operations.

    Primitive Types:
    - Signed Integers: i8, i16, i32, i64, i128
    - Unsigned Integers: u8, u16, u32, u64, u128
    - Floating Point Numbers: f32, f64
    - Boolean: bool
    - Character: char

    Compound Types:
    - Tuple: (i32, f64, char)
    - Array: [i32; 5]
    - Slice: &[i32]
    - String: String
    - Note: Rust does not have a built-in string type; it uses the String type from the standard library.

    Mutability:
    - All variables are immutable by default. Use 'mut' keyword to make any variable mutable.


    To compile and run this Rust file:
    $ rustc TVar.rs
    $ ./TVar

*/

fn main() {
    // Signed integers of different sizes
    // i8, i16, i32, i64, i128
    let a: i8 = -10;
    let b: i16 = -200;
    let c: i32 = -30000;
    let d: i64 = -4000000;
    let e: i128 = -5000000000;
    println!("Signed integers: {}, {}, {}, {}, {}", a, b, c, d, e);

    // Unsigned integers of different sizes
    // u8, u16, u32, u64, u128
    let f: u8 = 10;
    let g: u16 = 200;
    let h: u32 = 30000;
    let i: u64 = 4000000;
    let j: u128 = 5000000000;
    println!("Unsigned integers: {}, {}, {}, {}, {}", f, g, h, i, j);

    // Floating point numbers
    // f32, f64
    let k: f32 = 3.14;
    let l: f64 = 2.718281828459045;
    println!("Floating point numbers: {}, {}", k, l);

    // Boolean
    let m: bool = true;
    let n: bool = false;
    println!("Booleans: {}, {}", m, n);

    // Character
    let o: char = 'R';
    let p: char = '🦀';
    println!("Characters: {}, {}", o, p);

    // Tuple
    let q: (i32, f64, char) = (42, 3.14, 'R');
    println!("Tuple: {:?}", q); // Debug print
    let (x, y, z) = q; // Destructuring
    println!("Destructured Tuple: {}, {}, {}", x, y, z);
    println!("Accessed Tuple element: {}", q.0); // Accessing elements

    // Array
    let mut r: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", r); // Debug print
    let first_element = r[0]; // Accessing elements
    let n: usize = r.len(); // Array length
    r[2] = 10; // Modifying an element
    let s: [i32; 5] = [0; 5]; // Array of 5 elements initialized to 0
    println!("Initialized Array: {:?}", s);

    // Slice
    let t: &[i32] = &r; // Reference to an array (slice)
    println!("Array Slice: {:?}", t);

    // String
    let mut u: String = String::from("hello");
    u.push_str(", Rust!");
    println!("String: {}", u);

    // mutable vs immutable
    let v: i32 = 10; // Immutable by default
                     //v += 20; // This would cause a compile-time error
    let mut w: i32 = 10; // Mutable variable
    println!("Before update: {}", w);
    w += 20; // This is allowed
    println!("Immutable: {}, Mutable: {}", v, w);

    // Type inference
    let b = 10; // Rust infers this as i32
    let c = 3.14; // Rust infers this as f64
    println!("Inferred types: {}, {}", b, c);

    // Type casting
    let d: i32 = 10;
    let e: f64 = d as f64; // Casting i32 to f64
    println!("Type casting: {}", e);

    // Basic arithmetic operations
    let sum = 5 + 10; // Addition
    let difference = 95.5 - 4.3; // Subtraction
    let product = 4 * 30; // Multiplication
    let quotient = 56.7 / 32.2; // Division
    let remainder = 43 % 5; // Modulus
    println!(
        "Arithmetic operations: {}, {}, {}, {}, {}",
        sum, difference, product, quotient, remainder
    );

    // Compound assignment operators
    let mut f = 5;
    f += 10; // f = f + 10
             //f++; // Rust does not support ++ operator
             //f--; // Rust does not support -- operator
    f -= 2; // f = f - 2
    f *= 3; // f = f * 3
    f /= 4; // f = f / 4
    f %= 3; // f = f % 3
    println!("Compound assignment result: {}", f);
}
