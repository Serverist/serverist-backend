use common::domain::models::{UserId, IconId};
use uuid::Uuid;

pub struct User {
    id: UserId,
    account_id: AccountId,
    name: Name,
    description: Description,
    icon_id: IconId,
}

pub struct UserError;

impl User {
    pub fn new(id: UserId, account_id: AccountId, name: Name, description: Description, icon_id: IconId) -> Result<Self, UserError> {
        Ok(Self { id, account_id, name, description, icon_id })
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn icon_id(&self) -> &IconId {
        &self.icon_id
    }
}

pub struct AccountId(Uuid);

impl AccountId {
    pub fn new(id: Uuid) -> Self {
        AccountId(id)
    }
}

pub struct Description(String);

pub struct DescriptionError;

impl Description {
    pub fn new(description: String) -> Result<Self, DescriptionError> {
        if description.chars().count() <= 255 {
            Ok(Self(description))
        } else {
            Err(DescriptionError)
        }
    }
}

pub struct Name(String);

pub struct NameError;

impl Name {
    pub fn new(name: String) -> Result<Self, NameError> {
        if name.chars().count() <= 31 {
            Ok(Self(name))
        } else {
            Err(NameError)
        }
    }
}