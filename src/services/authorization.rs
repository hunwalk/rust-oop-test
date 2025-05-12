use crate::models::user::UserModel;
pub(crate) use crate::traits::authorization::Authorization;
use sha2::{Sha256, Digest};

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

impl Authorization for UserModel {
    fn login(&mut self, username: &str, password: &str) -> bool {
        let input_hash = hash_password(password);
        // Assume user.user.password is the hashed password
        if self.user.username == username && self.user.password == input_hash {
            self.authenticated = true;
        }
        self.authenticated
    }

    fn logout(&mut self) {
        self.authenticated = false;
    }

    fn is_authenticated(&self) -> bool {
        self.authenticated
    }
}