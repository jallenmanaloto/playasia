#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub address: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub name: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;
    settings.try_deserialize()
}

impl ApplicationSettings {
    pub fn connection_string(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

impl DatabaseSettings {
    pub fn database_name(&self) -> String {
        self.name.clone()
    }
}
