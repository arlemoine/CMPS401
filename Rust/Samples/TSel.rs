use std::io;

fn main() {

    // simple if
    let num1: i128 = 5;
    let num2: i128 = 9;

    if num1 < num2 {
        println!("{} is less than {}", num1, num2);
    }


    // if-else
    println!("How old are you?");
    let mut age: String = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");
    let age: i32 = age.trim().parse::<i32>().expect("Please enter a number") as i32;


    if age < 13 {
        println!("You are a child.");
    }
    else if age < 18 {
        println!("You are a teenager.");
    }
    else {
        println!("You are an adult.");
    }

    // if as expression
    let is_even: bool = if age % 2 == 0 { true } else { false };
    println!("Is your age even? {}", is_even);


    // match statement
    println!("Enter a number between 1 and 5:");
    let mut number: String = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: i32 = number.trim().parse::<i32>().expect("Please enter a number") as i32;

    match number {
        1 => println!("You entered one."),
        2 => println!("You entered two."),
        3 => println!("You entered three."),
        4 => println!("You entered four."),
        5 => println!("You entered five."),
        _ => println!("Number out of range!"),
    }

}