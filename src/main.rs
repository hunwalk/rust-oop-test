mod models;
mod structures;
mod services;
mod traits;

use std::io::{self, Write};
use rpassword::read_password;
use sha2::{Sha256, Digest};
use structures::user::User;
use models::user::UserModel;
use services::authorization::Authorization;

fn main() {
    let mut hasher = Sha256::new();
    hasher.update("password123".as_bytes());
    let hashed_password = format!("{:x}", hasher.finalize());

    let user = User {
        username: "hunwalk".to_string(),
        password: hashed_password,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    let mut user_model = UserModel {
        user,
        authenticated: false,
    };

    let mut username = String::new();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    if user_model.login(username, &password) {
        println!("Login successful! Welcome, {}.", user_model.user.first_name);
    } else {
        println!("Login failed. Invalid username or password.");
    }
}