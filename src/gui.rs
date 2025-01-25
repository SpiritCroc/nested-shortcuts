use crate::menu::{MenuEntry, MenuAction};

use iced::{
    Subscription, Length,
    widget::{button, text, Column},
    keyboard::Key,
    keyboard,
};

use std::process::{Command, exit};

pub struct MenuWidget {
    entries: Vec<MenuEntry>,
    sub_menu: Option<Box<MenuWidget>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    MenuClick { action: MenuAction },
    ShortcutKey { key: String },
    EscKey,
}

impl MenuWidget {
    pub fn new(entries: Vec<MenuEntry>) -> MenuWidget {
        MenuWidget {
            entries,
            sub_menu: None
        }
    }

    pub fn view(&self) -> Column<Message> {
        if let Some(menu) = &self.sub_menu {
            return menu.view();
        }
        Column::with_children(
            self.entries.iter().map(|e| {
                button(text(e.name.clone())).on_press(
                    Message::MenuClick { action: e.action.clone() }
                ).into()
            }).collect::<Vec<_>>()
        )
            .width(Length::Shrink)
            .height(Length::Shrink)
    }

    fn close_leaf_menu(&mut self) -> bool {
        if let Some(ref mut sub_menu) = self.sub_menu {
            if !sub_menu.close_leaf_menu() {
                self.sub_menu = None;
            }
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::MenuClick { action } => {
                match action {
                    MenuAction::Program { exec } => {
                        Command::new("sh")
                            .arg("-c")
                            .arg(exec)
                            .spawn()
                            .expect("Failed to spawn program");
                        exit(0);
                    }
                    MenuAction::SubMenu { entries } => {
                        self.sub_menu = Some(
                            Box::new(
                                MenuWidget::new(entries)
                            )
                        );
                    }
                }
            }
            Message::ShortcutKey { ref key } => {
                if let Some(sub_menu) = self.sub_menu.as_deref_mut() {
                    sub_menu.update(message);
                    return;
                }
                for entry in self.entries.clone().iter() {
                    if entry.shortcut == Some(key.to_string()) {
                        self.update(
                            Message::MenuClick { action: entry.action.clone() }
                        );
                    }
                }
            }
            Message::EscKey => {
                if !self.close_leaf_menu() {
                    exit(0);
                }
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, _|
            match key {
                Key::Character(c) => Some(Message::ShortcutKey { key: c.to_string() }),
                Key::Named(keyboard::key::Named::Escape) => Some(Message::EscKey),
                _ => None,
            }
        )
    }
}
