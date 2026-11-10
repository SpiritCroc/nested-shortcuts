use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuEntry {
    #[serde(deserialize_with = "deserialize_string")]
    pub title: String,
    #[serde(default, deserialize_with = "deserialize_optional_string", skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<String>,
    #[serde(flatten)]
    pub action: MenuAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_hint_offset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MenuAction {
    Program {
        #[serde(deserialize_with = "deserialize_string")]
        exec: String,
    },
    SubMenu { entries: Vec<MenuEntry> },
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StringScalar {
    String(String),
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    Bool(bool),
}

impl StringScalar {
    fn into_string(self) -> String {
        match self {
            Self::String(value) => value,
            Self::Signed(value) => value.to_string(),
            Self::Unsigned(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::Bool(value) => value.to_string(),
        }
    }
}

fn deserialize_string<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    StringScalar::deserialize(deserializer).map(StringScalar::into_string)
}

fn deserialize_optional_string<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    Option::<StringScalar>::deserialize(deserializer)
        .map(|value| value.map(StringScalar::into_string))
}

pub fn build_menu_from_file(path: &String) -> anyhow::Result<Vec<MenuEntry>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result = serde_yml::from_reader(reader)?;
    Ok(result)
}
