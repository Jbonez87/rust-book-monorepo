pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: i128
}

pub fn run() -> User {
    let user1 = User {
        email: String::from("John@john.com"),
        username: String::from("JCNYC123"),
        active: true,
        sign_in_count: 5,
    };
    return user1;
}

pub fn mutable_user() -> User {
    let mut user1: User = User {
        email: String::from("John@john.com"),
        username: String::from("JCNYC123"),
        active: true,
        sign_in_count: 5,
    };
    user1.email = String::from("Jc@jc.com");
    println!("{}", user1.email);
    return user1;
}

pub fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 5,
    }
}

pub fn inject_user() -> User {
    let user1 = User {
        email: String::from("John@john.com"),
        username: String::from("JCNYC123"),
        active: true,
        sign_in_count: 5,
    };
    let user2: User = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    return user2;
}

pub fn spread_inject_user() -> User {
    let user1 = User {
        email: String::from("John@john.com"),
        username: String::from("JCNYC123"),
        active: true,
        sign_in_count: 5,
    };
    let user2: User = User {
        email: String::from("another@example.com"),
        ..user1
    };
    return user2;
}

pub struct Color(i64, i64, i64);
pub struct Point(i64, i64, i64);

pub fn colors_points() -> Vec<(Color, Point)> {
    let black: Color = Color (0, 0, 0);
    let origin: Point = Point (0, 0, 0);
    return vec![(black, origin)];   
}
