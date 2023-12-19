use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::{
    prelude::{ElementWaitable, ElementQueryable, WebDriverError},
    By, DesiredCapabilities, WebDriver, WebElement, fantoccini::elements,
    error::WebDriverResult
};
use futures::future::try_join_all;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let driver = initialize_driver().await?;
    let url = Url::parse("https://propelplm.my.site.com/helpcenter/s/article/Propel-User-Guide")?;
    
    driver.goto(&url).await?;
    let elem = driver.query(By::XPath("//div/div[1]/div/div/div[2]/span/div/h1/span/span/span/span/span/span/span")).first().await?;
    // let elem = driver.query(By::XPath("//div/div[1]/div/div/div[2]/span/div/h2/span/a|//div/div[1]/div/div/div[2]/span/div/h2/span/span/a")).first().await?;
    elem.wait_until().displayed().await?;
    
    // print!("Element found: {}", elem.text().await?);

    let homepage_elements = get_homepage_elements(&driver).await?;
    // println!("homepage_elements: {:?}", homepage_elements);
    let homepage_element_links = get_element_links_absolute(&url, homepage_elements).await?;
    // let homepage_element_links = homepage_elements.iter().map(|element| get_element_links(element)).collect::<Vec<_>>();
    // let homepage_element_futured = try_join_all(homepage_element_links).await?;
    // println!("homepage_element_futured: {:?}", homepage_element_futured);
    // // now go to each of the sub pages linked to the homepage and recursively scrape them
    // for homepage_element_links in homepage_elements {
        
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
    print!("getting homepage elements");
    let elements = driver.find_all(By::XPath("//div/div[1]/div/div/div[2]/span/div/h2/span/a|//div/div[1]/div/div/div[2]/span/div/h2/span/span/a")).await?;
    // print!("elements: {:?}", elements);
    Ok(elements)
}

async fn get_element_links(element: &WebElement) -> Result<Option<String>, WebDriverError> {
    // println!("element link: {:?}", element.attr("href").await?);
    element.attr("href").await
}

async fn get_element_links_absolute(base_url: &Url, elements: Vec<WebElement>) -> Result<Vec<Result<Url, url::ParseError>>, Box<dyn Error>> {
    let element_links = elements.iter().map(|element| get_element_links(element)).collect::<Vec<_>>();
    let element_links = try_join_all(element_links).await?;
    let element_links = element_links.iter().map(|element_link| base_url.join(element_link.clone().unwrap().as_str())).collect::<Vec<_>>();
    
    Ok(element_links)
}