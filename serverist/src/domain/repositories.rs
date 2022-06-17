use crate::domain::models::server::Server;

pub struct ServerRepositoryError;

type ServerRepositoryResult<T> = std::result::Result<T, ServerRepositoryError>;

pub trait ServerRepository {
    fn insert(server: Server) -> ServerRepositoryResult<()>;
    fn update(server: Server) -> ServerRepositoryResult<()>;
    fn delete(server: Server) -> ServerRepositoryResult<()>;
}

use crate::domain::models::user::User;

pub struct UserRepositoryError;

type UserRepositoryResult<T> = std::result::Result<T, UserRepositoryError>;

pub trait UserRepository {
    fn insert(User: User) -> UserRepositoryResult<()>;
    fn update(User: User) -> UserRepositoryResult<()>;
    fn delete(User: User) -> UserRepositoryResult<()>;
}