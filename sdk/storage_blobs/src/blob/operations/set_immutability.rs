use crate::prelude::*;
use azure_core::{headers::*, RequestId, Response};
use time::OffsetDateTime;
use std::convert::{TryFrom, TryInto};

operation! {
    SetImmutability,
    client: BlobClient,
    retain_until_date: RetainUntilDate,
    ?mode: ImmutabilityMode
}

impl SetImmutabilityBuilder {
    pub fn into_future(mut self) -> SetImmutability {
        Box::pin(async move {
            let mut url = self.client.url()?;
            url.query_pairs_mut().append_pair("comp", "immutabilityPolicies");

            let mut headers = Headers::new();
            headers.add(self.retain_until_date);

            let mut request = BlobClient::finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            response.try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetImmutabilityResponse {
    pub request_id: RequestId,
    pub client_request_id: Option<String>,
    pub version: String,
    pub retain_until_date: OffsetDateTime,
    pub immutability_mode: String,
}

impl TryFrom<Response> for SetImmutabilityResponse {
    type Error = azure_core::Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        let headers = response.headers();
        Ok(SetImmutabilityResponse {
            request_id: request_id_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
            version: version_from_headers(headers)?,
            retain_until_date: date_from_headers(headers)?,
            immutability_mode: immutability_mode_from_headers(headers)?,
        })
    }
}
