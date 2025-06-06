use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_apex_processor_token_create`].

On request success, this will return a [`ProcessorTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorApexProcessorTokenCreateRequest {
    pub access_token: String,
    pub account_id: String,
}
impl FluentRequest<'_, ProcessorApexProcessorTokenCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorApexProcessorTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ProcessorTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/apex/processor_token/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "account_id" : self.params.account_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create Apex bank account token

Used to create a token suitable for sending to Apex to enable Plaid-Apex integrations.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn processor_apex_processor_token_create(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> FluentRequest<'_, ProcessorApexProcessorTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: ProcessorApexProcessorTokenCreateRequest {
                access_token: access_token.to_owned(),
                account_id: account_id.to_owned(),
            },
        }
    }
}
