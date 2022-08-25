use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct CredentialsBuilder {
    pub credentials: Credentials,
}

#[derive(Clone, Debug, Default)]
pub struct Credentials(pub Vec<Credential>);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Credential {
    email: String,
    password: String,
}

impl CredentialsBuilder {
    pub fn new() -> Self {
        Self {
            credentials: Credentials(Vec::new()),
        }
    }

    pub fn add_credential(mut self, email: String, password: String) -> Self {
        self.credentials.0.push(Credential { email, password });
        self
    }

    pub fn build(self) -> Credentials {
        self.credentials
    }
}

impl Credential {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
