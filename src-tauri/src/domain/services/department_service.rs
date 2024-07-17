struct DepartmentService<'a> {
    user_repo: &'a dyn UserRepository,
    department_repo: &'a dyn DepartmentRepository,
}

impl<'a> DepartmentService<'a> {
    pub fn new(user_repo: &'a dyn UserRepository, department_repo: &'a dyn DepartmentRepository) -> Self {
        Self { user_repo, department_repo }
    }

    pub fn register_user(&self, name: String, email: String) -> Result<User, String> {
        let user = User::new(name, email);
        self.user_repo.save(&user);
        Ok(user)
    }

    pub fn create_department(&self, name: String, manager: User) -> Result<Department, String> {
        let department = Department::new(name, manager);
        self.department_repo.save(&department)?;
        Ok(department)
    }

    pub fn add_team_member(
        &self,
        department_id: DepartmentId,
        member: User,
    ) -> Result<Department, String> {
        if let Some(mut department) = self.department_repo.find_by_id(&department_id) {
            department.add_team_member(member.clone());
            self.department_repo.save(&department)?;
            Ok(department)
        } else {
            Err(String::from("Invalid department_id."))
        }
    }

    pub fn remove_team_member(&self, department_id: DepartmentId, member_id: UserId) -> Result<Department, String> {
        if let Some(mut department) = self.department_repo.find_by_id(&department_id) {
            department.remove_team_member(&member_id)?;
            self.department_repo.save(&department)?;
            Ok(department)
        } else {
            Err(String::from("Invalid department_id."))
        }
    }
}
