struct InMemoryDepartmentRepositoryy {
    departments: HashMap<DepartmentId, Department>,
}

impl DepartmentRepository for InMemoryDepartmentRepositoryy {
    fn save(&self, department: &Department) {
        self.departments
            .insert(department.id.clone(), department.clone());
    }

    fn find_by_id(&self, id: &DepartmentId) -> Option<Department> {
        self.departments.get(id).cloned()
    }

    fn get_departments() -> Option<Vec<Department>> {
        let departments = self.departments.lock().unwrap();
        departments.values().cloned().collect()
    }
}
