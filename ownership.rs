fn main() {
    let x = 5;
    let y = x;

    println!("x: {x}");
    println!("y: {y}");
    // both variables are valid

    let s1 = String::from("hello");
    println!("s1: {s1}");
    let s2 = s1; // ownership of "hello" transferred to s2
                 // s1 is moved to s2

    // println!("s1: {s1}"); // no longer valid since the ownership is given to s2;
    // in rust only one variable can be the owner of an object (heap allocated memory)
    // when we do let s2 = s1, ownership of the string "hello" is given to s2 and s1 is no longer valid
    println!("s2: {s2}");

    // if you want to explicitly copy the string instead of moving, use the clone method
    let str1 = String::from("copied");
    let str2 = str1.clone();

    println!("str1: {str1}");
    println!("str2: {str2}");

    // moving happens even when it is passed to a function as arguements
    // when it is passed as an arguement, the variable in the function will take its ownership
    // the memory will be freed(dropped) when the function ends as the variable will go out of scope
    let s = String::from("Hi");
    take_ownership(s);
    // s is no longer valid here as the ownership was transferred to the function called (take_ownership)
    // println!("s: {s}") // no longer valid as the function takes over the ownership and frees the memory

    // values are moved even during function returns
    let s2 = String::from("ownership");
    let (s2, len) = calculate_length(s2);
    println!("THe length of the string \'{s2}\' is {len}")
}

fn take_ownership(str: String) {
    println!("Took ownership of string {str}")
} // memory of str is freed(dropped) here

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length) // this tuple is returned
}
