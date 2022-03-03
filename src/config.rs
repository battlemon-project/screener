use serde::{Deserialize};
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use tokio::sync::OnceCell;

use crate::Result;

pub fn set_up_browser() -> Result<FirefoxPreferences> {
    let preferences = FirefoxPreferences::new();
    Ok(preferences)
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    host: String,
    port: u16,
}

impl ApplicationSettings {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize)]
pub struct WebDriverSettings {
    address: String,
    port: u16,
    headless: bool,
}

impl WebDriverSettings {
    pub fn url(&self) -> String {
        format!("http://{}:{}", self.address, self.port)
    }

    pub fn headless(&self) -> bool {
        self.headless
    }
}

#[derive(Deserialize)]
pub struct ConstructorSettings {
    address: String,
}
impl ConstructorSettings {
    pub fn url(&self) -> String {
        format!("http://{}", self.address)
    }
}

#[derive(Deserialize)]
pub struct IpfsSettings {
    address: String,
    port: u16,
}
impl IpfsSettings {
    pub fn url(&self) -> String {
        format!("http://{}:{}", self.address, self.port)
    }
}

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub webdriver: WebDriverSettings,
    pub constructor: ConstructorSettings,
    pub ipfs: IpfsSettings,
}

pub fn load_config() -> std::result::Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;
    settings.try_into()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`",
                other
            )),
        }
    }
}

static CONFIG: OnceCell<Settings> = OnceCell::const_new();

pub async fn get_config() -> &'static Settings {
    CONFIG
        .get_or_init(|| async { load_config().expect("Couldn't load config") })
        .await
}
