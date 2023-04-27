use config::{Config, File};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let builder = Config::builder().add_source(File::with_name("configuration"));

    let config = builder.build()?;
    config.try_deserialize()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.username, self.password, self.host, self.database_name
        )
    }
}
