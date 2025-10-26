#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub enum Locale {
    #[default]
    En,
    It,
}

#[allow(dead_code)]
impl Locale {
    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::It => "it",
        }
    }

    pub fn all() -> &'static [Locale] {
        &[Locale::En, Locale::It]
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)]
pub struct Translations {
    #[serde(flatten)]
    map: HashMap<String, String>,
}

#[allow(dead_code)]
impl Translations {
    pub fn load(locale: Locale) -> anyhow::Result<Self> {
        let path = format!("./locales/{}.json", locale.as_str());
        let content = fs::read_to_string(path)?;
        let translations: Translations = serde_json::from_str(&content)?;
        Ok(translations)
    }

    pub fn get(&self, key: &str) -> String {
        self.map.get(key).cloned().unwrap_or_else(|| format!("MISSING_TRANSLATION: {}", key))
    }
}
