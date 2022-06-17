use common::domain::models::UserId;
use super::models::{ definitive, temporary };

pub struct DefinitiveUserRepositoryError;

type DefinitiveUserRepositoryResult<T> = std::result::Result<T, DefinitiveUserRepositoryError>;

pub trait DefinitiveUserRepository {
    fn insert(self, user: &definitive::User) -> DefinitiveUserRepositoryResult<()>;
    fn update(self, user: &definitive::User) -> DefinitiveUserRepositoryResult<()>;
    fn delete(self, id: &UserId) -> DefinitiveUserRepositoryResult<()>;
}

pub struct TemporaryUserRepositoryError;

type TemporaryUserRepositoryResult<T> = std::result::Result<T, DefinitiveUserRepositoryError>;

pub trait TemporaryUserRepository {
    fn insert(self, user: &temporary::User) -> DefinitiveUserRepositoryResult<()>;
    fn update(self, user: &temporary::User) -> DefinitiveUserRepositoryResult<()>;
    fn delete(self, id: &UserId) -> DefinitiveUserRepositoryResult<()>;
}