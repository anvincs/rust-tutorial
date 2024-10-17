fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // Output: The value of x is 5

    // cannot do since variables are immutable by default
    // x = 6;
    // println!("The value of x is: {x}");

    let mut y = 10; // the variable is declared as mutable
    println!("The value of y is {y}");
    // Output: The value of y is 10
    y = 8;
    println!("The value of y is {y}");
    // Output: The value of y is 8

    // Shadowing
    let z = 5;
    let z = 6;
    // we declared z again
    // the new value of z will overshadow the old value until it goes out of scope
    println!("The value of z is {z}");
    // Output: The value of z is 6
    {
        let z = z * 2;
        println!("The value of z in the inner loop is {z}");
        // Output: The value of z in the inner loop is 12
    }

    // we can even change dtatatype when shadowing
    let z = "Hello";
    println!("The value of z now is {z}");
    // Output: The value of z now is Hello
}
