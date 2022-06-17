pub struct Email(String);

pub struct EmailError;

impl Email {
    pub fn new(email: String) -> Result<Self, EmailError> {
        if validator::validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(EmailError)
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub struct Password(String);

pub struct PasswordError;

impl Password {
    pub fn new(password: String) -> Result<Self, PasswordError> {
        let regex = fancy_regex::Regex::new(r##"^(?=.*[a-zA-Z])(?=.*[0-9])[a-zA-Z0-9!"#$%&'()*+,\-./:;<=>?\[\\\]^_`{|}~]{8,}$"##).unwrap();
        if regex.is_match(&password).unwrap() {
            Ok(Password(password))
        } else {
            Err(PasswordError)
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}



pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(password_hash: String) -> Self { Self(password_hash) }
    pub fn value(&self) -> &str { &self.0 }
}