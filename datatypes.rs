fn main() {
    let num = 5;
    println!("Num: {num}");

    let x = 2.0; // f64
    println!("x: {x}");

    let y: f32 = 3.0; // f32
    println!("y: {y}");

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

    let t = true;
    println!("t: {t}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("heart_eyed_cat: {heart_eyed_cat}");

    // Compound types

    // tuples
    // tuples in rust have fixed size and cannot grow or shrink
    // elements can have different datatypes
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1); this is also possible
    let (p, q, r) = tup; // destructuring
    println!("q: {q}");

    // we can also access values in a tuple using indices
    let five_hundred = tup.0;
    println!("five_hundred: {five_hundred}");

    // Array
    // arrays in rust have fixed size and can only store values of same datatype

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // type annotation is done in square brackets with type semicolon no. of elements
    // let a = [1, 2, 3, 4, 5];

    // elements in an array can be accessed via indices
    let first = a[0];
    let second = a[1];
    println!("first: {first}");
    println!("second: {second}");
}
