use crate::menu::{MenuEntry, MenuAction};

use iced::{
    Subscription, Length,
    widget::{
        button,
        rich_text,
        span,
        Column,
    },
    keyboard::Key,
    keyboard,
    window,
    time::Instant,
};

use std::process::{Command, exit};

pub struct MenuWidget {
    entries: Vec<MenuEntry>,
    sub_menu: Option<Box<MenuWidget>>,
    // If true, run exec, else print the command to execute to pipe into some other tool
    exec_standalone: bool,
    start_time: Option<Instant>,
}

#[derive(Debug, Clone)]
pub enum Message {
    MenuClick { action: MenuAction },
    ShortcutKey { key: String },
    NavigateBack,
    Exit,
}

impl From<MenuAction> for Message {
    fn from(item: MenuAction) -> Self {
        Message::MenuClick { action: item }
    }
}

impl MenuEntry {
    fn label(&self) -> iced::widget::text::Rich<Message> {
        if let Some(c) = &self.shortcut {
            let hint_offset = &self.shortcut_hint_offset.unwrap_or_default();
            let index = if *hint_offset <= 0 {
                self.title.to_lowercase().find(&c.to_lowercase())
            } else {
                self.title.to_lowercase().match_indices(&c.to_lowercase()).nth(*hint_offset).map(|(i, _)| i)
            };
            if let Some(index) = index {
                if index < self.title.len() - 1 {
                    rich_text([
                        span(&self.title[0..index]),
                        span(&self.title[index..index+1]).underline(true),
                        span(&self.title[index+1..]),
                    ])
                } else {
                    rich_text([
                        span(&self.title[0..index]),
                        span(&self.title[index..]).underline(true),
                    ])
                }
            } else {
                rich_text([
                    span(&self.title),
                    span(" ("),
                    span(c).underline(true),
                    span(")"),
                ])
            }
        } else {
            rich_text([
                span(&self.title),
            ])
        }
    }
}

impl MenuWidget {
    pub fn new(entries: Vec<MenuEntry>, exec_standalone: bool, start_time: Option<Instant>) -> MenuWidget {
        MenuWidget {
            entries,
            sub_menu: None,
            exec_standalone,
            start_time,
        }
    }

    pub fn view(&self) -> Column<Message> {
        if let Some(menu) = &self.sub_menu {
            return menu.view();
        }
        let result = Column::with_children(
            self.entries.iter().map(|e| {
                button(e.label()).on_press(
                    e.action.clone().into()
                )
                    .style(button::text)
                    .width(Length::Fill)
                    .into()
            }).collect::<Vec<_>>()
        )
            .width(Length::Fill)
            .height(Length::Shrink);
        if let Some(start) = &self.start_time {
            println!("Menu rendered after {:?}", start.elapsed());
        }
        result
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
                        if self.exec_standalone {
                            eprintln!("Launch: {exec}");
                            Command::new("sh")
                                .arg("-c")
                                .arg(exec)
                                .spawn()
                                .expect("Failed to spawn program");
                        } else {
                            print!("{exec}");
                        }
                        exit(0);
                    }
                    MenuAction::SubMenu { entries } => {
                        eprintln!("Navigate to submenu");
                        self.sub_menu = Some(
                            Box::new(
                                MenuWidget::new(entries, self.exec_standalone, None)
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
                        return;
                    }
                }
                eprintln!("Ignore key without active shortcut");
            }
            Message::NavigateBack => {
                eprintln!("Navigate back");
                if !self.close_leaf_menu() {
                    exit(0);
                }
            }
            Message::Exit => {
                eprintln!("Exit");
                exit(0);
            }
        }
    }

    fn keyboard_subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, _|
            match key {
                Key::Character(c) => Some(Message::ShortcutKey { key: c.to_string() }),
                Key::Named(keyboard::key::Named::Escape) => Some(Message::NavigateBack),
                Key::Named(keyboard::key::Named::Backspace) => Some(Message::NavigateBack),
                _ => None,
            }
        )
    }

    fn window_event_subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, _id| {
            if let iced::event::Event::Window(window::Event::Unfocused) = event {
                Some(Message::Exit)
            } else {
                None
            }
        })
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.keyboard_subscription(),
            self.window_event_subscription(),
        ])
    }
}
