use crate::prelude::*;
use azure_core::{headers::*, RequestId, Response};
use std::convert::{TryFrom, TryInto};

operation! {
    SetLegalHold,
    client: BlobClient,
    legal_hold: LegalHold,
}

impl SetLegalHoldBuilder {
    pub fn into_future(mut self) -> SetLegalHold {
        Box::pin(async move {
            let mut url = self.client.url()?;
            url.query_pairs_mut().append_pair("comp", "legalhold");

            let mut headers = Headers::new();
            headers.add(self.legal_hold);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            response.try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetLegalHoldResponse {
    pub request_id: RequestId,
    pub client_request_id: Option<String>,
    pub version: String,
    pub legal_hold: bool,
}

impl TryFrom<Response> for SetLegalHoldResponse {
    type Error = azure_core::Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        let headers = response.headers();
        Ok(SetLegalHoldResponse {
            request_id: request_id_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
            version: version_from_headers(headers)?,
            legal_hold: legal_hold_from_headers(headers)?,
        })
    }
}