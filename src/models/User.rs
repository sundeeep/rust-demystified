use serde::{Serialize, Deserialize};

pub enum UserRole{
    ADMIN,
    USER,
    CEO,
    CFO,
    CTO,
    COO
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub displayName: String,
    pub email: String,
    pub password: String,
    pub role: UserRole
}