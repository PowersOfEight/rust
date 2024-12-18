struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);


fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color: black -> red = {}", black.0);

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let arr = [1,2,3,4,5];

    user1.email = String::from("anotheremail@example.com");

    let user2 = User{
        email: String::from("another@example.com"),
        ..user1
    };
}
