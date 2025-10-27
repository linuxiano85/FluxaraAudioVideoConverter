#![allow(unused_imports)]
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, fs, sync::RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    En,
    It,
}

impl Default for Language {
    fn default() -> Self {
        Language::En
    }
}

#[allow(dead_code)]
impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::It => "it",
        }
    }

    pub fn all() -> &'static [Language] {
        &[Language::En, Language::It]
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

static CURRENT_LANGUAGE: Lazy<RwLock<Language>> = Lazy::new(|| RwLock::new(Language::En));
static TRANSLATIONS: Lazy<RwLock<Translations>> = Lazy::new(|| {
    let lang = *CURRENT_LANGUAGE.read().unwrap();
    let translations = Translations::load(lang).unwrap_or_default();
    RwLock::new(translations)
});

pub fn set_language(lang: Language) {
    *CURRENT_LANGUAGE.write().unwrap() = lang;
    let new_translations = Translations::load(lang).unwrap_or_default();
    *TRANSLATIONS.write().unwrap() = new_translations;
}

pub fn translate(key: &str) -> String {
    TRANSLATIONS.read().unwrap().get(key)
}

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)]
pub struct Translations {
    #[serde(flatten)]
    map: HashMap<String, String>,
}

#[allow(dead_code)]
impl Translations {
    pub fn load(language: Language) -> anyhow::Result<Self> {
        let path = format!("./locales/{}.json", language.as_str());
        let content = fs::read_to_string(path)?;
        let translations: Translations = serde_json::from_str(&content)?;
        Ok(translations)
    }

    pub fn get(&self, key: &str) -> String {
        self.map.get(key).cloned().unwrap_or_else(|| format!("MISSING_TRANSLATION: {}", key))
    }
}
