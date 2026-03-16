use chrono::{DateTime, Utc};

pub struct User {
    id: u64,
    display_name: String,
    role: Role,
    created_at: DateTime<Utc>,
}

pub enum Role {
    User,
    Admin,
}

pub struct NewEmailUser {
    pub display_name: String,
    pub email: String,
    pub password: String,
}
