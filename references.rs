fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // a reference is passed and the value itself is not moved
    // hence the ownership remains with the len variable itself
    println!("The length of '{s1}' is {len}.");

    // let str = String::from("Hello");
    // change(&str); // cause error since references are immutable and change function tries to change it

    let mut str = String::from("Hello");
    modify(&mut str); // mutable reference is passed and hence the function can modify the value of str
    println!("str: {str}");

    let r1 = &mut str;
    println!("r1: {r1}");
    // if you have a mutable reference to a value, you can have no other mutable references to that value.
    // We also cannot have a mutable reference while we have an immutable one to the same value.

    // println!("{}, {}", r1, r2);
    // let r2 = &mut str;
    // trying to create a reference to a value which already has a mutable reference

    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // // you cannot create a mutable reference if one or more immutable references exists
    // // will cause error
    // println!("{}, {}, and {}", r1, r2, r3);

    // A reference’s scope starts from where it is introduced and continues through the last time that reference is used.

    let mut s = String::from("hello");
    // multiple immutable reference is allowed

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point
    // therefore their scope ends here

    let r3 = &mut s; // no problem since r1 and r2 is no longer in scope
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // since s is only a reference to a value, it wont be freed(dropped) after the function returns

// fn change(s: &String) {
//     // references are immutable by default and hence cannot be modified
//     s.push_str(", world");
// }

fn modify(s: &mut String) {
    // the arguement is a mutable reference to a string
    s.push_str(", world!");
}
