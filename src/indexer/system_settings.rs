use serde::{Deserialize, Serialize};

use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::panic;
use std::path::Path;

#[derive(Serialize, Debug, Deserialize)]
pub struct SystemSettings {
    pub port: String,
    pub ignore_domains: Vec<String>,
    pub indexer_enabled: bool,
    pub ignore_strings: Vec<String>,
}

impl Default for SystemSettings {
    fn default() -> Self {
        SystemSettings {
            port: "7172".to_string(),
            ignore_strings: vec![],
            indexer_enabled: false,
            ignore_domains: vec![
                ".lvh.me".to_string(),
                "//lvh.me".to_string(),
                "//localhost/".to_string(),
                "//localhost:".to_string(),
                "google.com/".to_string(),
                "youtube.com/".to_string(),
                "ebay.com/".to_string(),
                "aha.io/".to_string(),
                "newrelic.com/".to_string(),
                "datadoghq.com/".to_string(),
                "amazon.com/".to_string(),
                "woot.com/".to_string(),
                "imgur.com".to_string(),
                "gstatic.com/".to_string(),
                "slack.com".to_string(),
                "facebook.com".to_string(),
                "instagram.com".to_string(),
                "pintrest.com".to_string(),
                "zillow.com".to_string(),
                "redfin.com".to_string(),
            ],
        }
    }
}

pub fn write_settings(config: &SystemSettings) {
    let path = Path::new(super::BASE_INDEX_DIR.as_str());
    let path_name = path.join("server_settings.toml");
    super::create_directory(&super::BASE_INDEX_DIR);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path_name)
        .expect("setting file write");
    file.write_all(toml::to_string(&config).unwrap().as_bytes())
        .expect("file");
    file.sync_all().expect("file write");
}

pub fn read_settings() -> SystemSettings {
    let path = Path::new(super::BASE_INDEX_DIR.as_str());
    let path_name = path.join("server_settings.toml");
    super::create_directory(&super::BASE_INDEX_DIR);
    let mut s = String::new();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path_name);

    match file {
        Err(_why) => {
            //println!("couldn't open {}: {}", path_name, why.to_string());
        }
        Ok(mut file) => {
            if let Err(why) = file.read_to_string(&mut s) {
                panic!("couldn't read {:#?}: {}", path_name.to_str(), why)
            };
        }
    };
    if !s.is_empty() {
        let config: SystemSettings = toml::from_str(&s).expect("bad config parse");
        config
    } else {
        SystemSettings::default()
    }
}
