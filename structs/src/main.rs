struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    output_user(&user1);

    user1.email = String::from("anotheremail@example.com");
    output_user(&user1);

    output_user(&build_user(String::from("someone@example.com"), String::from("someusername123")));

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    output_user(&user2);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    output_user(&user2);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email:String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        sign_in_count: 2,
    }
}

fn output_user(user:&User) {
    println!("email:{}, username:{}, active:{}, sign_in_count:{}",
        user.email, user.username, user.active, user.sign_in_count);
}