use azure_core::headers::{self, Header, LEGAL_HOLD};

#[derive(Debug, Clone)]
pub struct LegalHold(bool);

impl From<bool> for LegalHold {
    fn from(status: bool) -> Self {
        Self(status)
    }
}

impl Header for LegalHold {
    fn name(&self) -> headers::HeaderName {
        LEGAL_HOLD
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}