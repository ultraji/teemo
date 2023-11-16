use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use log::{error, info, warn};
use serde::{Serialize, Deserialize};
use thirtyfour::{Capabilities, WebDriver};

use crate::action::Action;

#[derive(Deserialize, Serialize)]

pub struct WebDriverParams {
    #[serde(rename = "url", default)]
    pub server_url: String,
    #[serde(rename = "capabilities", default)]
    pub capabilities: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Step {
    #[serde(flatten)]
    pub action: Action,
    #[serde(rename = "allowSkip", default)]
    pub allow_skip: bool,
    #[serde(rename = "resourceName", default)]
    pub resource_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Teemo {
    #[serde(rename = "webDriver")]
    pub web_driver: WebDriverParams,
    pub steps: Vec<Step>,
    pub content: String,
}

impl Teemo {
    pub fn new(filename: &str) -> Result<Self, anyhow::Error> {
        let config = fs::read_to_string(filename)?;

        let teemo: Teemo = serde_yaml::from_str(&config)?;

        Ok(teemo)
    }


    pub async fn run(self) -> Result<(), anyhow::Error> {
        let mut map: HashMap<String, String> = HashMap::new();

        let caps: Capabilities = serde_json::from_str(&self.web_driver.capabilities)?;
        let driver = WebDriver::new(&self.web_driver.server_url, caps).await?;

        for item in self.steps {
            match item.action.run(&driver).await {
                Ok(result) => {
                    if item.resource_name != "" {
                        info!("Run {:?} success and get {}: {}", item.action, item.resource_name, result);
                        map.insert(item.resource_name.clone(), result);
                    } else {
                        info!("Run {:?} success.", item.action)
                    }
                },
                Err(err) => {
                    if item.allow_skip {
                        warn!("Run {:?} failed: {} but allow it failed.", item.action, err);
                    } else {
                        error!("Run {:?} failed: {}.", item.action, err);
                        return Err(anyhow::Error::from(err));
                    }
                }
            };
        }

        let mut result_str = self.content.clone();
        for (k, v) in map {
            result_str = result_str.replace(&format!("${{{}}}", k), &v);
        }
        println!("{}", result_str);

        driver.quit().await?;

        Ok(())
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_init() {
        let contents = r#"
webDriver:
  url: 'http://localhost:9515'
  capabilities: |
    {
      "browserName": "chrome",
      "goog:chromeOptions": {
        "args": ["--headless", "--window-size=1920,1080"]
      }
    }
steps:
- action: Goto
  url: https://google.com
- action: Click
  locator: {strategy: XPath, expr: '//*[@id="L2AGLb"]'}
  allowSkip: true
- action: Wait
  timeout: 3
- action: SendKeys
  value: [ 'ultraji', 'Key::Enter']
  locator: { strategy: XPath, expr: '//*[@id="APjFqb"]' }
- action: Wait
  timeout: 3
- action: GetElemText
  locator: { strategy: XPath, expr: '//*[@id="rso"]/div[1]/div/div/div[1]/div/div/span/a/div/div/div/cite' }
  resourceName: first_item
content: "This is first search result item of ultraji. It's ${first_item}. "
"#;
        fs::write("teemo.yaml", contents).expect("Failed to write config file");

        let teemo = Teemo::new("teemo.yaml").expect("Failed to read test config file");
        assert_eq!(teemo.steps[0].action, Action::Goto { url: "https://google.com".to_string() });
    }
}