#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String, // TODO: will make this secure later
    created_at: String,
    updated_at: String,
    deleted_at: String,
}

impl User {
    pub fn new(
        id: i32,
        name: String,
        email: String,
        password: String,
        created_at: String,
        updated_at: String,
        deleted_at: String,
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
