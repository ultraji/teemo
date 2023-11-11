use serde::{Serialize, Deserialize};
use thirtyfour::{By, WebDriver, WebElement};
use thirtyfour::error::WebDriverResult;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag="strategy", content="expr")]
pub enum Locator {
    XPath(String),
    ClassName(String),
    Id(String),
    Name(String),
    Css(String),
}

impl Locator {
    pub async fn find_with(&self, driver: &WebDriver) -> WebDriverResult<WebElement> {
        match self {
            Locator::XPath(value) => driver.find(By::XPath(value)).await,
            Locator::ClassName(value) => driver.find(By::ClassName(value)).await,
            Locator::Id(value) => driver.find(By::Id(value)).await,
            Locator::Name(value) => driver.find(By::Name(value)).await,
            Locator::Css(value) => driver.find(By::Css(value)).await,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let elem = Locator::XPath("//*[@id=\"199\"]/img".to_string());

        let elem_str = serde_yaml::to_string(&elem).expect("Failed to serialize");
        assert_eq!(elem_str, "strategy: XPath\nexpr: //*[@id=\"199\"]/img\n");

        let elem: Locator = serde_yaml::from_str(&elem_str).expect("Failed to deserialize");
        assert_eq!(elem, Locator::XPath("//*[@id=\"199\"]/img".to_string()));
    }
}