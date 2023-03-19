use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:4444", caps).await?;
     
     driver.goto("https://app.cookieyes.com/dashboard").await?;
     // Get the cookies for the website
    let cookies = driver.get_cookies().await?;

    // Print the cookies
    for cookie in &cookies {
        let result = cookie;
        println!("Name: {}, Value: {}", result.name(), result.value());
    }
     driver.quit().await?;

     Ok(())
}