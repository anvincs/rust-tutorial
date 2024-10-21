enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // Option is a built in enum in rust (Option<T>)
    // it can be a value of the specified type or None
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        // these are called match arms
        // it has two parts, a patten and some code
        // => operator seperates the pattern and the code
        // here it returns an integer

        // match is exhaustive
        // match must cover all possibilities
        // we cannot exclude any
        // eg: we cannot exclude Quarter and keep only Penny, Nickel, and Dime
    }
}

fn main() {
    let quarter = Coin::Quarter;
    println!(
        "The value of quarter in cents is {}",
        value_in_cents(quarter)
    );
    let penny = Coin::Penny;
    println!("The value of penny in cents is {}", value_in_cents(penny));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => do_something(other),
        // other handles all the possibilities we haven't explicitly handled (all integers except 3 and 7)
        // other contains the value
        _ => do_something(), // _ also handles all the remaining possibilities but does not give us the value
                             // this is useful when the function to be performes on remaining possibilities does not need its value
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // Here we only have to match only one pattern
    // we handle the pattern in the match
    // and for other pattern we add _ => () after processing just one variant
    // this is annoying boilerplate code

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // the if let syntax allows us to handle one pattern
    // it works similar to match
    // it handles the provided pattern and ignores all others
    // this results in less code but loses the exhaustive checking of match

    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // }

    // the above code(match) has the same functionality as the one below(if let)

    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {state:?}!");
    // } else {
    //     // handles all other patterns
    //     // similar to _ => () in match
    //     count += 1;
    // }
}

fn add_fancy_hat() {
    println!("Added fancy hat");
}

fn remove_fancy_hat() {
    println!("REmoved fancy hat");
}

fn do_something() {
    println!("Did something");
}
