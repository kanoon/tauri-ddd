struct InMemoryUserRepository {
    users: HashMap<UserId, User>,
}

impl UserRepository for InMemoryUserRepository {
    fn save(&self, user: &User) {
        self.users.insert(user.id.clone(), user.clone());
    }

    fn find_by_id(&self, id: &UserId) -> Option<User> {
        self.users.get(id).cloned()
    }

    fn get_users() -> Option<Vec<User>> {
        let users = self.users.lock().unwrap();
        users.values().cloned().collect()
    }
}
