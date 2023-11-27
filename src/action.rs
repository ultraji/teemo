use std::path::Path;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use tokio::time::{sleep, Duration};
use thirtyfour::prelude::*;

use crate::util;
use crate::locator::Locator;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag="action")]
pub enum Action {
    Goto{url: String},
    Wait{timeout: u64},
    SendKeys{value: Vec<String>, locator: Locator},
    Click{locator: Locator},
    Clear{locator: Locator},
    GetElemText{locator: Locator},
    ExecScript{script: String, params: Vec<Value>},
    ExecAsyncScript{script: String, params: Vec<Value>},
    TakeScreenshot,
    TakeElemScreenshot{locator: Locator},
}

impl Action {
    pub async fn run(&self, driver: &WebDriver) -> Result<String, anyhow::Error> {
        let mut result = "".to_string();

        match &self {
            // Action::LoadJavaScripts {url} => {
            //     let load_script = format!("{}{}{}",
            //         "var teemoScript = document.createElement(\"script\");",
            //         format!("teemoScript.src = \"{}\";", url),
            //         "document.head.appendChild(teemoScript);");
            //     driver.execute(&load_script, vec![]).await?
            // }
            Action::Goto {url} => driver.goto(url).await?,
            Action::Wait {timeout} => sleep(Duration::from_secs(*timeout)).await,
            Action::SendKeys {value, locator} => locator.find_with(driver).await?.send_keys(util::gen_key_content_from_input(value)).await?,
            Action::Click {locator} => locator.find_with(driver).await?.click().await?,
            Action::Clear {locator} => locator.find_with(driver).await?.clear().await?,
            Action::GetElemText {locator} => result = locator.find_with(driver).await?.text().await?,
            Action::ExecScript {script, params} => {
                let ret = driver.execute(script, params.clone()).await?;
                result = format!("{}", ret.json().to_string());
            },
            Action::ExecAsyncScript {script, params} => {
                let ret = driver.execute_async(script, params.clone()).await?;
                result = format!("{}", ret.json().to_string());
            },
            Action::TakeScreenshot => {
                let filename = util::gen_random_filename() + ".png";
                driver.screenshot(Path::new(&filename)).await?;
                result = filename
            }
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