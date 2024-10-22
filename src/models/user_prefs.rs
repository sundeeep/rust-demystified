use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Theme{
    DARK,
    LIGHT,
    SYSTEM
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPreferences{
    pub theme:Theme
}