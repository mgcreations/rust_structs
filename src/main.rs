fn main() {
    let user1 = User {
        email: String::from("info@mgcreations.co.za"),
        username: String::from("MG Creations"),
        active: true,
        sign_in_count: 1,
    };

    println!("{:?}", user1);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
