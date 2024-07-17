use crate::domain::value_objects::Email;
use crate::domain::value_objects::UserId;
use uuid::Uuid;

pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: Email,
}

impl User {
    pub fn new(name: String, email: Email) -> Self {
        Self {
            id: UserId(Uuid::new_v4()),
            name,
            email,
        }
    }
}
