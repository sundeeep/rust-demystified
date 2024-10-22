use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum UserRole{
    ADMIN,
    USER,
    CEO,
    CFO,
    CTO,
    COO
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole
}