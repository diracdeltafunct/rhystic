use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String, // TODO: will make this secure later
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: i32,
        name: String,
        email: String,
        password: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> User {
        User {
            id,
            name,
            email,
            password,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}
