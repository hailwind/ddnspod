use anyhow::Result;
use dnspod_lib::prelude::*;
use dnspod_lib::response::*;

pub struct Client {
    secret_id: String,
    secret_key: String,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(secret_id: String, secret_key: String) -> Self {
        Self {
            secret_id,
            secret_key,
            client: reqwest::blocking::Client::new(),
        }
    }
    pub fn execute(&self, request: impl ExtractCommonParams) -> Result<Response> {
        let secret_id = self.secret_id.as_str();
        let secret_key = self.secret_key.as_str();

        let client = &self.client;

        let url = request.url();
        let body = request.body();
        let headers = request.headers(&secret_id, &secret_key);
        let headers = (&headers).try_into()?;

        let request = client.post(url).headers(headers).body(body).build()?;

        let res: Response = client.execute(request)?.json()?;

        if res.Response.Error.is_some() {
            let err = dnspod_lib::serde_json::to_string_pretty(&res)?;
            return Err(anyhow::anyhow!("{}", err));
        }

        Ok(res)
    }
}
