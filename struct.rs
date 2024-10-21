struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

// To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        username: String::from("someuser123"),
        sign_in_count: 1,
    };

    println!("name: {}", user1.username);
    // if the instance is mutable, we can modify the user
    // the entire instance must be mutable
    // marking only certain fields as mutable is not allowed
    user1.username = String::from("newuser123");
    println!("name: {}", user1.username);

    let user2 = build_user(
        String::from("seconduser123"),
        String::from("secondone@example.com"),
    );

    println!(
        "Second user, name = {} and email = {}",
        user2.username, user2.email
    );

    // let user3 = User{
    //     name: user1.name,
    //     username: user1.username,
    //     sign_in_count: user1.sign_in_count,
    //     email: String::from("newemail@example.com")
    // };
    let user3 = User {
        email: String::from("newemail@example.com"),
        ..user1 // similar to spread operator in javascript
    };
    // same as the above commented code but with lesser more readable code
    println!(
        "Third user, name = {} and email = {}",
        user3.username, user3.email
    );

    // We can no longer use user1 as a whole as the username field form user1 is moved to user3

    // println!("User 1 name: {}", user1.username); // will cause error

    // we can still use the unmoved parts of user1 individually
    println!("User1 email: {}", user1.email);

    // if we had given new username and email to the user3, we could have still used user1 as a whole
    // as the other two fields are integers and they are copied not moved

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples,
    // and when naming each field as in a regular struct would be verbose or redundant.

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!(
        "The color code of black is ({}, {}, {})",
        black.0, black.1, black.2
    );

    println!(
        "The coordinates of origin are: ({}, {}, {})",
        origin.0, origin.1, origin.2
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 35,
        height: 60,
    };

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1 ? {}", rect2.can_hold(&rect1));
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // same as username: username
        email,    // same as email: email
        sign_in_count: 1,
    }
}
