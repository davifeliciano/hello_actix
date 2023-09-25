use actix_web::{error, web, HttpResponse};
use deadpool_postgres;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
    pub port: Option<u16>,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;

        config.try_deserialize()
    }

    pub fn json_extractor_config() -> web::JsonConfig {
        web::JsonConfig::default().error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::UnprocessableEntity().into())
                .into()
        })
    }
}
