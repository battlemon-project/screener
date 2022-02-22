use thirtyfour::By;

use crate::Result;

pub async fn save_png(driver: &thirtyfour::WebDriver) -> Result<()> {
    driver
        .get("https://www.clicktorelease.com/tools/CubemapToEquirectangular/index-managed.html")
        .await?;
    let element = driver.find_element(By::Id("capture")).await?;
    element.click().await?;

    Ok(())
}
