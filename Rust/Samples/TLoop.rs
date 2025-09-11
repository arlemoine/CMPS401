fn main() {
    // loop

    let mut count = 0;
    // infinite loop with break
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }
    println!("Loop exited after {} iterations", count);


    // while
    let mut num = 10;

    while num > 0 {
        println!("Countdown: {}", num);
        num -= 1;
    }
    println!("Liftoff!");

    // for

    // 1, 2, ..., 9
    // inclusive..exclusive
    for i in 1..10 {
        if i % 2 == 0 {
            println!("Even number: {}", i);
        } else {
            println!("Odd number:  {}", i);
        }
    }

    // 1, 2, ..., 10
    // inclusive..inclusive
    for i in 1..=10 {
        println!("Inclusive: {}", i);
    }

    // reversed
    // 9, 8, ..., 0
    for i in (0..10).rev() {
        println!("Reversed: {}", i);
    }

    // for each 
    let nums = [10, 20, 30, 40, 50];
    for num in nums.iter() {
        println!("Array element: {}", num);
    }


}