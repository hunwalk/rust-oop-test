pub struct User {
    pub username: String,
    pub password: String, // store the hashed password here
    pub first_name: String,
    pub last_name: String,
}

pub struct UserModel {
    pub user: User,
    pub authenticated: bool,
}