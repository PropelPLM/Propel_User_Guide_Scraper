use serde::Serialize;
use std::{error::Error, io::Write};
use std::thread;
use std::time::Duration;
use thirtyfour::{
    prelude::{ElementWaitable, ElementQueryable, WebDriverError},
    By, DesiredCapabilities, WebDriver, WebElement, fantoccini::elements,
    error::WebDriverResult
};
use futures::future::try_join_all;
use url::Url;
use std::fs;
use std::io;
use serde::ser::StdError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let driver = initialize_driver().await?;
    let url = Url::parse("https://propelplm.my.site.com/helpcenter/s/article/Propel-User-Guide")?;
    
    driver.goto(&url).await?;
    let elem = driver.query(By::XPath("//div/div[1]/div/div/div[2]/span/div/h1/span/span/span/span/span/span/span")).first().await?;
    elem.wait_until().displayed().await?;

    println!("homepage displayed");

    let homepage_elements = get_homepage_elements(&driver).await?;
    let homepage_element_links = get_element_links_absolute(&url, homepage_elements).await?;
    println!("homepage element urls generated");

    // write the pages into 
    for link in homepage_element_links {
        write_page_content_to_file(&driver, link).await?;
        // let link = link?;
        // driver.goto(&link).await?;
        // let heading_element = driver.query(By::XPath("//html/body/div[3]/div[2]/div/div[3]/div/div[2]/div[1]/article[1]/h1|/html/body/div[3]/div[2]/div/div[2]/div[1]/div/div[2]/div[1]/article[1]/h1")).first().await?;
        // heading_element.wait_until().displayed().await?;
        // let heading_text = heading_element.text().await?.replace(" ", "_");

        // let page_content_element = driver.query(By::XPath("//div/div[1]/div/div/div[2]/span/div")).first().await?;
        // let page_content_text = page_content_element.text().await?;
        // // get subpage reference links

        // println!("heading text: {:?}", heading_text);
        // let dest_file_name = format!("scraped_pages/{heading_text}.txt");
        // println!("dest_file_name: {:?}", dest_file_name);
        // fs::write(dest_file_name, page_content_text)?;
        println!("printing_subpage_links");
        let subpage_link_elements = get_subpage_elements(&driver).await?;
        let subpage_links = get_element_links_absolute(&url, subpage_link_elements).await?;
        for subpage_link in subpage_links {
            write_page_content_to_file(&driver, subpage_link).await?;
        }
    }


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
    // print!("getting homepage elements");
    let elements = driver.find_all(By::XPath("//div/div[1]/div/div/div[2]/span/div/h2/span/a|//div/div[1]/div/div/div[2]/span/div/h2/span/span/a")).await?;
    // print!("elements: {:?}", elements);
    Ok(elements)
}

async fn get_subpage_elements(driver: &WebDriver) -> Result<Vec<WebElement>, WebDriverError> {
    // finds all the subpage elements that link to different pages of the guide
    // print!("getting homepage elements");
    // let elements = driver.find_all(By::XPath("/html/body/div[3]/div[2]/div/div[3]/div/div[2]/div[1]/article[2]/div/div/div/div/div[1]/div/div/div[2]/span/div/ul/li/span/span/a")).await?;
    let elements = driver.find_all(By::XPath("//div/div[1]/div/div/div[2]/span/div/ul/li/span/span/a")).await?;
    print!("sub__elements: {:?}", elements);
    Ok(elements)
}

async fn get_element_links(element: &WebElement) -> Result<Option<String>, WebDriverError> {
    println!("element link: {:?}", element.attr("href").await?);
    element.attr("href").await
}

async fn get_element_links_absolute(base_url: &Url, elements: Vec<WebElement>) -> Result<Vec<Result<Url, url::ParseError>>, Box<dyn Error>> {
    let element_links = elements.iter().map(|element| get_element_links(element)).collect::<Vec<_>>();
    let element_links = try_join_all(element_links).await?;
    let element_links = element_links.iter().filter(|element_link| match element_link {
        Some(_) => true,
        None => false
    }).map(|element_link| base_url.join(element_link.clone().unwrap().as_str())).collect::<Vec<_>>();
    
    Ok(element_links)
}

async fn write_page_content_to_file(driver: &WebDriver, page_element_link: Result<Url, url::ParseError>) -> Result<(), Box<dyn Error>>{
    let page_element_link = page_element_link?;
    driver.goto(&page_element_link).await?;
    // heading element tends to be fixed in the same place
    let heading_element = driver.query(By::XPath("//html/body/div[3]/div[2]/div/div[3]/div/div[2]/div[1]/article[1]/h1|/html/body/div[3]/div[2]/div/div[2]/div[1]/div/div[2]/div[1]/article[1]/h1")).first().await?;
    heading_element.wait_until().displayed().await?;
    let heading_text = heading_element.text().await?.replace(" ", "_");

    let page_content_element = driver.query(By::XPath("//div/div[1]/div/div/div[2]/span/div")).first().await?;
    let page_content_text = page_content_element.text().await?;
    // get subpage reference links

    println!("heading text: {:?}", heading_text);
    let dest_file_name = format!("scraped_pages/{heading_text}.txt");
    println!("dest_file_name: {:?}", dest_file_name);
    fs::write(dest_file_name, page_content_text)?;
    Ok(())
}
