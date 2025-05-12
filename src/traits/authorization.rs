pub trait Authorization {
    fn login(&mut self, username: &str, password: &str) -> bool;
    fn logout(&mut self);
    fn is_authenticated(&self) -> bool;
}