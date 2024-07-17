pub trait UserRepository {
    fn save(&self, user: &User);
    fn find_by_id(&self, id: &UserId) -> Option<User>;
    fn get_users() -> Option<Vec<User>>;
}
