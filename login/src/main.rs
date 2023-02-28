use auth::{login, LoginAction, Role, DeniedReason, get_users, hash_password};

fn user_accepted(role: &Role) {
    println!("You are logged in as {role:?}");
}

fn main() {
    auth::build_users_file();
    println!("Welcome to the not very secure login system!");

    let users = get_users();

    let mut username = String::new();
    let mut password = String::new();
    let stdin = std::io::stdin();

    println!("Enter your username:");
    stdin.read_line(&mut username).expect("No keyboard!");
    println!("Enter your password:");
    stdin.read_line(&mut password).unwrap();
    
    match login(&users, &username, &password) {
        None => {
            println!("{} is not a known user", username.trim());
        },
        Some(login_action) => {
            login_action.do_login(user_accepted, |reason| {
                println!("Denied: {reason:?}");
            })
        }
        _ => {}
    }
}

fn on_denied(reason: &DeniedReason) {
    println!("Denied {reason:?}")
}