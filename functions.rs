fn main() {
    let x: i32 = 5;
    print_double(x);
    let fact_x = fact(x);
    println!("The factorial of {x} is {fact_x}");

    let y = {
        let x = 3;
        x + 1 // no semicolon and hence not a statement but an expression
    };

    println!("The value of y is: {y}");
}

fn print_double(x: i32) {
    let double = x * 2;
    println!("The double of {x} is {double}");
}

// functions with return types
// return type of the function is provided using an arrow ->
fn fact(x: i32) -> i32 {
    if x == 1 {
        return 1;
    }

    // return x * fact(x - 1);
    x * fact(x - 1) // no semicolon and hence not a statement but an expression

    // if not inside a block other than the function block itself, an expression without return keyword will
    // act as a return statement
}
