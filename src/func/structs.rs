#[derive(Debug)]
pub struct User {
    pub username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    pub fn print(&self) {
        println!("{}", self.username)
    }
}

impl User {
    pub fn dup(username: String, email: String, sign_in_count: u64, active: bool) -> User {
        User {
            username: username,
            email: email,
            sign_in_count: sign_in_count,
            active: active,
        }
    }
}

pub fn structs() -> User {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("aaaaa"),
        ..user1
    };
    user2
}
