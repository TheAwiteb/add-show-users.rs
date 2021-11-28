use promptly::prompt;

#[derive(Debug)]
struct User {
    id: usize,
    username: String,
    email: String,
}

impl User {
    fn new(id: usize, username: String, email: String) -> User {
        User {
            id,
            username,
            email,
        }
    }
}

fn add_user(user_id: usize) -> User {
    let username: String = prompt("👱 Username").expect("Cannot read line ❌");
    let email: String = prompt("📧 Email").expect("Cannot read line ❌");

    User::new(user_id, username, email)
}

fn show_users(users: &Vec<User>) {
    for (user_id, user) in users.iter().enumerate() {
        println!("\n===🆔=== User id {} ===🆔===\n", user_id + 1);
        println!("👱 Username: {}", user.username);
        println!("📧 Email: {}", user.email);
    }
}

fn main() {
    let mut user_list: Vec<User> = Vec::new();

    loop {
        let user_input: char =
            prompt("\n'q' for quit.\n'a' for add new user.\n's' for show users.\n>>")
                .expect("Cannot read line ❌");
        match user_input {
            'q' => break,
            'a' => {
                println!();
                user_list.push(add_user(user_list.len()))
            }
            's' => show_users(&user_list),
            _ => println!("Unknown input, try again.\n"),
        }
    }
}
