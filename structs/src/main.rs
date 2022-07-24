struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1username"),
        active: true,
        sign_in_count: 1,
    };

    let email = user1.email;
    user1.email = String::from("user1anothere@example.com");
    println!(
        "user1 prev email:{}, user1 cur email:{}",
        email, user1.email
    );

    let user2 = build_user(
        String::from("user2@example.com"),
        String::from("user2username"),
    );
    println!("user2 email:{}", user2.email);

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1
    };
    println!("user3 email:{}", user3.email);
    println!("user1 email:{}", user1.email);
    // error[E0382]: borrow of moved value: `user1.username`
    // println!("user1 username:{}", user1.username);

    let color = Color(0, 0, 0);
    let point = Point(0, 0, 0);
    println!("color[0]: {}", color.0);
    println!("point[0]: {}", point.0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
