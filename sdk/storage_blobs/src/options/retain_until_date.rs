
use azure_core::{Header, headers::{self, RETAIN_UNTIL_DATE, HeaderValue}, date};
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct RetainUntilDate(OffsetDateTime);

impl From<OffsetDateTime> for RetainUntilDate {
    fn from(date: OffsetDateTime) -> Self {
        Self(date)
    }
}

impl Header for RetainUntilDate {
    fn name(&self) -> headers::HeaderName {
        RETAIN_UNTIL_DATE
    }

    fn value(&self) -> headers::HeaderValue {
        HeaderValue::from(date::to_rfc1123(&self.0))
    }
}
