struct Department {
    id: DepartmentId,
    name: String,
    manager: User,
    teams: Vec<User>
}

impl<'a> Department<'a> {
    pub fn new(name: String, manager: User) -> Self {
        let user = user_repo.find_by_id(&user.id);
        Self {
            id: DepartmentId(Uuid::new_v4()),
            name: name.to_string(),
            manager: user.clone(),
            teams: Vec::new()
        }
    }

    pub fn add_team_member(&mut self, user: User) -> Result<(), String> {
        if self.members.iter().any(|x| &x.id == &user.id) {
            Err(String::from("Member already exists"))
        } else {
            self.members.push(user);
            Ok(())
        }
    }

    pub fn remove_team_member(&mut self, member_id: &UserId) -> Result<(), String> {
        if let Some(pos) = self.teams.iter().position(|x| &x.id == member_id) {
            self.teams.remove(pos);
            Ok(())
        } else {
            Err(String::from("Member not found"))
        }
    }
}
