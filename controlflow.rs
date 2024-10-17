fn main() {
    let age = 5;

    // if should have an expressoin that returns boolean
    // rust will not automatically convert non boolean types to boolean like in certain languages like javascript
    if age >= 18 {
        println!("You are eligible to vote")
    } else {
        println!("You are not eligible to vote")
    }

    // if else if

    let number = 6;

    if number % 4 == 0 {
        println!("number {number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("number {number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("number {number} is divisible by 2");
    } else {
        println!("number {number} is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // the returns (here 5 and 6) should be of same type
    // let n = if condition { 7 } else { 8.2 };
    // This will cause error since 7 is an integer and 8.2 is a floating-point number
    println!("The value of number is: {number}");

    // Loops

    // loop {
    //     println!("Again!");
    // } this is an infinite loop

    // Returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels
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

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // better approach (for loop)
    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        // (1..4) = [1,4)
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
