fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but there's no more string that
               // we could meaningfully use the value 5 with. word is now totally invalid!

    println!("word: {word}");

    // String slices
    // -------------

    // string slices are a reference to a part of the string
    let mut str = String::from("hello world");

    let word1 = &str[0..5]; // [0, 5)
    let word2 = &str[6..11]; // [6, 11)

    // [0..5] = [..5]
    // [6..len] = [6..] where len = str.len() (Length of the string)
    // [..] represents entire string

    println!("hello: {}", word1);
    println!("world: {}", word2);

    let wrd1 = first_word_slice(&str);
    println!("word1 = {wrd1}");

    str.clear();

    // println!("word1 = {wrd1}"); // wrd1 becomes invalid since the string str is cleared
    // will cause compile error

    let st = "This program is written in rust using zed editor";
    // The type of s here is &str.
    //  it’s a slice pointing to that specific point of the binary.
    // This is also why string literals are immutable. &str is an immutable reference.

    let st_wrd1 = first_word_slice(&st);
    println!("The first word is: {st_wrd1}");

    // Slices on arrays
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[2..4]; // slice = [3,4]
                            // this slice is of a type &[i32]
    assert_eq!(slice, &[3, 4]);
    // assert_eq!(slice, &[2, 3]); // assert will fail here
}

fn first_word(s: &String) -> usize {
    // returns the index where the first word in the string ends
    // ie return the index of the first occurence of a space

    let bytes = s.as_bytes(); // convert string into an array of bytes

    // an iterator is created using the iter method
    // iterator returns each element in a collection
    // enumerate wraps the result of iter and returns each element as part of a tuple instead.
    // the tuple contains the index and the reference to the value
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// fn first_word_slice(s: &String) -> &str {
fn first_word_slice(s: &str) -> &str {
    // this will now allow both &String ans &str values as parameters
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
