use thirtyfour::common::capabilities::firefox::FirefoxPreferences;

use crate::Result;

pub fn get_browser_settings() -> Result<FirefoxPreferences> {
    let mut preferences = FirefoxPreferences::new();
    preferences.set("browser.download.folderList", 2u64)?;
    preferences.set("browser.download.manager.showWhenStarting", false)?;
    preferences.set("browser.download.dir", "/screenshots")?;
    preferences.set("browser.helperApps.neverAsk.saveToDisk", "image/png")?;

    Ok(preferences)
}
