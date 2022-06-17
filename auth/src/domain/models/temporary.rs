use common::domain::models::{UserId, PastDateTime};

use super::common::{Email, PasswordHash};

pub struct User {
    id: UserId,
    created_at: PastDateTime,
    email: Email,
    hashed_password: PasswordHash,
}

impl User {
    pub fn new(id: UserId, created_at: PastDateTime, email: Email, hashed_password: PasswordHash) -> Self { Self { id, created_at, email, hashed_password } }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn hashed_password(&self) -> &PasswordHash {
        &self.hashed_password
    }
}