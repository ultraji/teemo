use std::path::Path;

use serde::{Serialize, Deserialize};
use tokio::time::{sleep, Duration};
use thirtyfour::prelude::*;

use crate::util;
use crate::locator::Locator;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag="action")]
pub enum Action {
    Goto{url: String},
    Wait{timeout: u64},
    TakeScreenshot,
    SendKeys{value: Vec<String>, locator: Locator},
    Click{locator: Locator},
    GetElemText{locator: Locator},
    TakeElemScreenshot{locator: Locator},
}

impl Action {
    pub async fn run(&self, driver: &WebDriver) -> Result<String, anyhow::Error> {
        let mut result = "".to_string();

        match &self {
            Action::Goto {url} => driver.goto(url).await?,
            Action::Wait {timeout} => sleep(Duration::from_secs(*timeout)).await,
            Action::TakeScreenshot => {
                let filename = util::gen_random_filename() + ".png";
                driver.screenshot(Path::new(&filename)).await?;
                result = filename
            }
            Action::SendKeys {value, locator} => locator.find_with(driver).await?.send_keys(util::gen_key_content_from_input(value)).await?,
            Action::Click {locator} => locator.find_with(driver).await?.click().await?,
            Action::GetElemText {locator} => {
                let elem = locator.find_with(driver).await?;
                result = elem.text().await?;
            },

            Action::TakeElemScreenshot {locator} => {
                let filename = util::gen_random_filename() + ".png";
                locator.find_with(driver).await?.screenshot(Path::new(&filename)).await?;
                result = filename;
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_and_deserialize() {
        let action = Action::Goto { url: "https://local.host".to_string() };
        let action_str = serde_yaml::to_string(&action).expect("Failed to serialize");

        assert_eq!(action_str, "action: Goto\nurl: https://local.host\n");

        let action: Action = serde_yaml::from_str(&action_str).expect("Failed to deserialize");
        assert_eq!(action, Action::Goto { url: "https://local.host".to_string() });
    }
}