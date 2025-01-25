#[derive(Debug, Clone)]
pub struct MenuEntry {
    pub name: String,
    pub shortcut: Option<String>,
    pub action: MenuAction,
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    Program { exec: String },
    SubMenu { entries: Vec<MenuEntry> },
}
