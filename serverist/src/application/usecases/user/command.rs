use async_trait::async_trait;
use uuid::Uuid;

pub struct UserCommandError;

type Result<T> = std::result::Result<T, UserCommandError>;

#[async_trait]
pub trait CreateUserCommand {
    async fn execute(id: Uuid, account_id: String, name: String, description: String, icon_id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait UpdateUserCommand {
    async fn execute(id: Uuid, account_id: String, name: String, description: String, icon_id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait DeleteUserCommand {
    async fn execute(id: Uuid) -> Result<()>;
}