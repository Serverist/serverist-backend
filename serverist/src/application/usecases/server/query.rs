use async_trait::async_trait;
use common::application::query::{OrderBy, Sort, Page};
use uuid::Uuid;

pub struct ServerQueryError;

pub struct User {
    id: Uuid,
    account_id: String,
    name: String,
    description: String,
    icon_id: Uuid,
}

pub struct ServerWithOwner {
    id: Uuid,
    name: String,
    owner: User,
    host: Option<String>,
    port: Option<u16>,
    description: String,
    icon_id: Uuid,
}

pub struct Server {
    id: Uuid,
    name: String,
    owner: Uuid,
    host: Option<String>,
    port: Option<u16>,
    description: String,
    icon_id: Uuid,
}

#[async_trait]
pub trait FindServersWithOwnerQuery {
    async fn execute(order_by: &OrderBy, sort: Sort, page: &Page) -> Result<Vec<ServerWithOwner>, ServerQueryError>;
}

#[async_trait]
pub trait FindServerWithOwnerQuery {
    async fn execute(id: Uuid) -> Result<ServerWithOwner, ServerQueryError>;
}

#[async_trait]
pub trait FindServersByOwnerQuery {
    async fn execute(id: Uuid) -> Result<Vec<Server>, ServerQueryError>;
}