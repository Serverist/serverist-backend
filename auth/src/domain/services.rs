use super::models::common::{Password, PasswordHash};

pub trait HashPasswordService {
    fn hash(password: Password) -> PasswordHash;
    fn verify(password: Password, hash: PasswordHash) -> bool;
}