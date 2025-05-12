use crate::structures::user::User;

pub struct UserModel {
    pub user: User,
    pub authenticated: bool,
}

impl UserModel {
    pub fn print_full_name(&self) {
        println!("{} {}", self.user.first_name, self.user.last_name);
    }
}