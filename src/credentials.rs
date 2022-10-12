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
    fn new() -> Self {
        Self { credentials: Credentials(Vec::new()) }
    }

    #[must_use]
    pub fn add_credential(mut self, email: String, password: String) -> Self {
        self.credentials.0.push(Credential { email, password });
        self
    }

    #[must_use]
    pub fn build(self) -> Credentials {
        self.credentials
    }
}

impl Credential {
    #[must_use]
    pub fn email(&self) -> &str {
        &self.email
    }

    #[must_use]
    pub fn password(&self) -> &str {
        &self.password
    }
}

impl Credentials {
    #[must_use]
    pub fn builder() -> CredentialsBuilder {
        CredentialsBuilder::new()
    }
}
