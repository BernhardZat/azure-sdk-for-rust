use azure_core::{Header, headers::{IMMUTABILITY_MODE, HeaderValue}};

#[derive(Debug, Clone)]
pub enum ImmutabilityMode {
    Unlocked,
    Locked,
}

impl Header for ImmutabilityMode {
    fn name(&self) -> azure_core::headers::HeaderName {
        IMMUTABILITY_MODE
    }

    fn value(&self) -> azure_core::headers::HeaderValue {
        match self {
            ImmutabilityMode::Unlocked => HeaderValue::from("unlocked"),
            ImmutabilityMode::Locked => HeaderValue::from("locked"),
        }
    }
}