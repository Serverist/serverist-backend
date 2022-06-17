use chrono::{DateTime, Utc, TimeZone};
use uuid::Uuid;
use common::domain::models::{UserId, IconId, PastDateTime};
use validator::Validate;
use validators::prelude::*;

#[derive(Debug)]
pub struct Server {
    id: Id,
    created_at: PastDateTime,
    updated_at: PastDateTime,
    name: Name,
    owner_id: UserId,
    host: Option<Host>,
    port: Option<u16>,
    description: Description,
    icon_id: IconId,
}

pub struct ServerError;

impl Server {
    pub fn new(id: Id, created_at: PastDateTime, updated_at: PastDateTime, name: Name, owner_id: UserId, host: Option<Host>, port: Option<u16>, description: Description, icon_id: IconId) -> Result<Self, ServerError> {
        let server = Self {
            id,
            created_at,
            updated_at,
            name,
            owner_id,
            host,
            port,
            description,
            icon_id
        };
        Ok(server)
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn created_at(&self) -> &PastDateTime {
        &self.created_at
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn owner_id(&self) -> &UserId {
        &self.owner_id
    }

    pub fn host(&self) -> Option<&Host> {
        self.host.as_ref()
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn set_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn set_host(&mut self, host: Option<Host>) {
        self.host = host;
    }

    pub fn set_port(&mut self, port: Option<u16>) {
        self.port = port;
    }

    pub fn set_description(&mut self, description: Description) {
        self.description = description;
    }

    pub fn icon_id(&self) -> &IconId {
        &self.icon_id
    }
}

impl PartialEq for Server {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Id(Uuid);

impl Id {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Name(String);

#[derive(Debug)]
pub struct  NameError;

impl Name {
    pub fn new(name: String) -> Result<Self, NameError> {
        if name.chars().count() <= 31 {
            Ok(Self(name))
        } else {
            Err(NameError)
        }
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Host {
    Ip(std::net::IpAddr),
    Domain(std::net::H)
}

#[derive(Debug)]
pub struct HostError;

impl Host {
    pub fn new(host: &str) -> Result<Self, HostError> {
        #[derive(Validate)]
        struct HostEntry {
            #[validate(ema)]
            host: String
        }
        if validators::url::Host::Domain() {
            Err(HostError)
        } else {
            Ok(Self(host.to_string()))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Description(String);

#[derive(Debug)]
pub struct DescriptionError;

impl Description {
    pub fn new(description: &str) -> Result<Self, DescriptionError> {
        if description.chars().count() <= 256 {
            Err(DescriptionError)
        } else {
            Ok(Description(description.to_string()))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_ok() {
        Name::new("a".repeat(31)).unwrap();
    }

    #[test]
    #[should_panic]
    fn name_err() {
        Name::new("a".repeat(32)).unwrap();
    }

    #[test]
    fn host_ok_1() {
        Host::new("192.168.0.1").unwrap();
    }

    #[test]
    fn host_ok_2() {
        Host::new("example.com").unwrap();
    }

    #[test]
    #[should_panic]
    fn host_err_1() {
        Host::new("192.168.0").unwrap();
    }

    #[test]
    #[should_panic]
    fn host_err_2() {
        Host::new("exmaple.1").unwrap();
    }
}