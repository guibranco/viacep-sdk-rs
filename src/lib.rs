use log::debug;
use serde::Deserialize;
use std::io;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct ZipCode {
    #[serde(rename = "cep")]
    pub zip: String,
    #[serde(rename = "logradouro")]
    pub address: String,
    #[serde(rename = "complemento")]
    pub complement: String,
    #[serde(rename = "bairro")]
    pub neighborhood: String,
    #[serde(rename = "localidade")]
    pub city: String,
    #[serde(rename = "uf")]
    pub state_initials: String,
    #[serde(rename = "unidade")]
    pub unit: String,
    pub ibge: String,
    pub gia: String,
}

fn to_io_error<E>(err: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::other(err)
}

struct UriMaker {
    api_base: String,
}

impl UriMaker {
    pub fn new(api_base: String) -> UriMaker {
        UriMaker { api_base }
    }

    fn build_url(&self, path: &str) -> Result<Url, url::ParseError> {
        let url = Url::parse(&self.api_base)?.join(path)?;
        Ok(url)
    }

    pub fn get_by_zipcode(&self, zip_code: &str) -> Url {
        self.build_url(&format!("ws/{}/json", zip_code)).unwrap()
    }

    pub fn get_by_address(&self, state_initials: &str, city: &str, address: &str) -> Url {
        self.build_url(&format!("ws/{}/{}/{}/json", state_initials, city, address))
            .unwrap()
    }
}

pub struct ViaCepClient {
    uri_maker: UriMaker,
    http: reqwest::Client,
    runtime: tokio::runtime::Runtime,
}

impl ViaCepClient {
    pub fn new() -> ViaCepClient {
        let http = reqwest::Client::new();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let uri_maker = UriMaker::new("https://viacep.com.br/".to_owned());
        ViaCepClient {
            uri_maker,
            http,
            runtime,
        }
    }

    async fn get_json(&self, url: Url) -> Result<serde_json::Value, io::Error> {
        debug!("GET {}", url);
        let response = self
            .http
            .get(url)
            .send()
            .await
            .map_err(to_io_error)?;
        debug!("Response: {}", response.status());
        let value: serde_json::Value = response.json().await.map_err(to_io_error)?;
        Ok(value)
    }

    pub fn get_zipcode(&self, zip_code: &str) -> Result<Option<ZipCode>, io::Error> {
        let url = self.uri_maker.get_by_zipcode(zip_code);
        self.runtime.block_on(async {
            let value = self.get_json(url).await?;
            let result: Option<ZipCode> = serde_json::from_value(value).map_err(to_io_error)?;
            Ok(result)
        })
    }

    pub fn search(
        &self,
        state_initials: &str,
        city: &str,
        address: &str,
    ) -> Result<Option<Vec<ZipCode>>, io::Error> {
        let url = self.uri_maker.get_by_address(state_initials, city, address);
        self.runtime.block_on(async {
            let value = self.get_json(url).await?;
            let result: Option<Vec<ZipCode>> =
                serde_json::from_value(value).map_err(to_io_error)?;
            Ok(result)
        })
    }
}

impl Default for ViaCepClient {
    fn default() -> Self {
        Self::new()
    }
}
