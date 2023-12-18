use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::{
    prelude::{ElementWaitable, ElementQueryable, WebDriverError},
    By, DesiredCapabilities, WebDriver, WebElement, fantoccini::elements,
};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let driver = initialize_driver().await?;
    let url = Url::parse("https://propelplm.my.site.com/helpcenter/s/article/Propel-User-Guide")?;
    
    driver.goto(url).await?;
    // let elem = driver.query(By::XPath("//span[text()='Propel User Guide']")).first().await?;
    let elem = driver.query(By::XPath("//div/div[1]/div/div/div[2]/span/div")).first().await?;
    elem.wait_until().displayed().await?;
    
    print!("Element found: {}", elem.text().await?);

    // let homepage_elements = get_homepage_elements(&driver).await?;

    // for homepage_element in homepage_elements {

    // }


    driver.quit().await?;
    Ok(())
}

pub fn scrape_homepage() {

}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}

async fn get_homepage_elements(driver: &WebDriver) -> Result<Vec<WebElement>, WebDriverError> {
    // finds all the homepage elements that link to different pages of the guide
    let elements = driver.find_all(By::XPath("//div/div[1]/div/div/div[2]/span/div/h2/span/a|//div/div[1]/div/div/div[2]/span/div/h2/span/span/a")).await?;
    Ok(elements)
}