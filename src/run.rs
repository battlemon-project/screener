use thirtyfour::WebDriver;

use crate::config;
use crate::routes::save_png;
use crate::Result;

pub async fn run() -> Result<()> {
    let driver = get_web_driver().await?;
    save_png(&driver).await?;

    driver.quit().await?;
    Ok(())
}

pub async fn get_web_driver() -> Result<WebDriver> {
    let browser_settings = config::get_browser_settings()?;
    let mut capabilities = thirtyfour::DesiredCapabilities::firefox();
    capabilities.set_preferences(browser_settings)?;
    let driver = thirtyfour::WebDriver::new("http://localhost:4444", &capabilities).await?;

    Ok(driver)
}
