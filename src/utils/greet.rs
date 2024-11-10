use crate::models::user::{User, UserRole};

fn welcome() {
  let user: User = User{
    username: "sundeeep".to_string(),
    display_name: "Sandeeep Dasari".to_string(),
    email: "sundeeepdev@gmail.com".to_string(),
    password: "drugboard.ai".to_string(),
    role: UserRole::ADMIN
  };
  
  println!("Hello, {}", user.email);
}