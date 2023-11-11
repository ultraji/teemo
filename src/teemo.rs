use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use log::{error, info, warn};
use serde::{Serialize, Deserialize};
use thirtyfour::{DesiredCapabilities, WebDriver};

use crate::action::Action;

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

        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:9515", caps).await?;

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
steps:
- action: Goto
  url: https://google.com
- action: Click
  locator: {strategy: XPath, expr: '//*[@id="L2AGLb"]'}
  alowSkip: true
- action: Wait
  timeout: 5
- action: SendKeys
  value: [ 'ultraji', 'Key::Enter']
  locator: { strategy: XPath, expr: '//*[@id="APjFqb"]' }
- action: Wait
  timeout: 5
- action: GetElemText
  locator: { strategy: XPath, expr: '//*[@id="rso"]/div[1]/div/div/div[1]/div/div/span/a/div/div/div/cite' }
  resourceName: first_item
- action: Wait
  timeout: 5
content: "This is first search result item of ultraji. It's ${first_item}. "
"#;
        fs::write("teemo.yaml", contents).expect("Failed to write config file");

        let teemo = Teemo::new("teemo.yaml").expect("Failed to read test config file");
        assert_eq!(teemo.steps[0].action, Action::Goto { url: "https://google.com".to_string() });
    }
}