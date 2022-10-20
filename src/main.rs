fn main() {
    println!("Hello, world! to Control Flow in Rust");
    let number = 3;
    // if else statement
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // if statement
    if number != 0 {
        println!("number was something other than zero");
    }
    // if else if else statement
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // if let statement
    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("The value of num is: {}", num);

    //loop statement
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break with return value
        }
    };

    println!("The result is {}", result);

    // while statement
    let mut number_ = 3;

    while number_ != 0 {
        println!("{}!", number_);

        number_ -= 1;
    }

    println!("LIFTOFF!!!");

    // for statement

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // for statement with range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // for statement with range and step

    for number in (1..4).rev().step_by(2) {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}
