pub trait DepartmentRepository {
    fn save(&self, department: &Department);
    fn find_by_id(&self, id: &DepartmentId) -> Option<Department>;
    fn get_departments() -> Option<Vec<Department>>;
}
