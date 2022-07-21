struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn display(&self) {
        println!("Username:{}, Email:{}, Sign In Count:{}, Active:{}", self.username, self.email, self.sign_in_count, self.active);
    }
    fn display2(&self) {
        println!("Username:{}, Email:{}, Sign In Count:{}, Active:{}", self.username, self.email, self.sign_in_count, self.active);
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.display();
    user1.display2();
}
