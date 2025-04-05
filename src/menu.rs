use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuEntry {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<String>,
    #[serde(flatten)]
    pub action: MenuAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_hint_offset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MenuAction {
    Program { exec: String },
    SubMenu { entries: Vec<MenuEntry> },
}

pub fn build_menu_from_file(path: &String) -> anyhow::Result<Vec<MenuEntry>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result = serde_yml::from_reader(reader)?;
    Ok(result)
}
