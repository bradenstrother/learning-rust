struct AlwaysEqual; // Unit Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let sq = Rectangle::square(3);

    println!("squared: {}", sq);

    println!("Can_hold: {}", rect.can_hold(&rect2));

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let new_user = build_user(String::from("this_user@email.com"), String::from("this_user"));

    println!("{}", new_user.active);
    println!("user2 Structure\nActive: {}\nUsername: {}\nEmail: {}\nSign in Count: {}",
             user2.active,
             user2.username,
             user2.email,
             user2.sign_in_count);
    println!("rect is {:#?}", rect);
    dbg!(&rect);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}

