use async_trait::async_trait;
use common::application::query::OrderBy;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserQueryError;

type Result<T> = std::result::Result<T, UserQueryError>;

pub struct User {
    id: Uuid,
    account_id: Uuid,
    name: String,
    description: String,
    icon_id: String,
}

impl User {
    pub fn new(id: Uuid, account_id: Uuid, name: String, description: String, icon_id: String) -> Self { Self { id, account_id, name, description, icon_id } }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub fn icon_id(&self) -> &str {
        self.icon_id.as_ref()
    }
}

pub struct Server {
    id: Uuid,
    name: String,
    host: Option<String>,
    port: Option<u16>,
    description: String,
    icon_id: Uuid,
}

impl Server {
    pub fn new(id: Uuid, name: String, host: Option<String>, port: Option<u16>, description: String, icon_id: Uuid) -> Self { Self { id, name, host, port, description, icon_id } }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn host(&self) -> Option<&String> {
        self.host.as_ref()
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub fn icon_id(&self) -> Uuid {
        self.icon_id
    }
}

pub struct UserWithServers {
    id: Uuid,
    account_id: Uuid,
    name: String,
    description: String,
    icon_id: String,
    servers: Vec<Server>,
}

impl UserWithServers {
    pub fn new(id: Uuid, account_id: Uuid, name: String, description: String, icon_id: String, servers: Vec<Server>) -> Self { Self { id, account_id, name, description, icon_id, servers } }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub fn icon_id(&self) -> &str {
        self.icon_id.as_ref()
    }

    pub fn servers(&self) -> &[Server] {
        self.servers.as_ref()
    }
}

#[async_trait]
pub trait FindUserByIdQuery {
    async fn execute(id: Uuid) -> Result<User>;
}
