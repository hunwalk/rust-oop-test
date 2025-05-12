use crate::structures::user::UserModel;  

impl UserModel {
    pub fn print_full_name(&self) {
        println!("{} {}", self.user.first_name, self.user.last_name);
    }
}