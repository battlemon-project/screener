use serde::Deserialize;
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;

use crate::Result;

pub fn get_browser_settings() -> Result<FirefoxPreferences> {
    let mut preferences = FirefoxPreferences::new();
    preferences.set("browser.download.folderList", 2u64)?;
    preferences.set("browser.download.manager.showWhenStarting", false)?;
    preferences.set("browser.download.dir", "/home/seluser/screenshots")?;
    preferences.set("browser.helperApps.neverAsk.saveToDisk", "image/png")?;

    Ok(preferences)
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct WebDriverSettings {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub webdriver: WebDriverSettings,
}

pub fn get_config() -> std::result::Result<Settings, config::ConfigError> {
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
