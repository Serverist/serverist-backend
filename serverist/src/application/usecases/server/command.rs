use async_trait::async_trait;
use uuid::Uuid;


pub struct ServerCommandError;

#[async_trait]
pub trait DeleteServerCommand {
    async fn execute(id: &Uuid) -> Result<(), ServerCommandError>;
}


pub struct CreateServerCommandError;

#[async_trait]
pub trait CreateServerCommand {
    async fn execute(
        id: &Uuid,
        owner_id: &Uuid,
        name: &str,
        host: Option<&str>,
        port: Option<&str>,
        description: Option<&str>
    ) -> Result<(), CreateServerCommandError>;
}

#[async_trait]
pub trait UpdateServerCommand {
    async fn execute(id: &Uuid, owner_id: &Uuid, name: &str, host: Option<&str>, port: Option<u16>, description: Option<&str>) -> Result<(), ServerCommandError>;
}