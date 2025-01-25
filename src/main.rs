mod menu;
mod gui;

use menu::{MenuEntry, MenuAction};
use gui::MenuWidget;

use iced::Color;

fn main() -> iced::Result {
    let window_settings = iced::window::Settings {
        resizable: false,
        decorations: false,
        transparent: true,
        exit_on_close_request: true,
        position: iced::window::Position::Centered,
        level: iced::window::Level::AlwaysOnTop,
        size: iced::Size::new(50.0, 80.0), // TODO
        ..Default::default()
    };
    iced::application("nmenu", MenuWidget::update, MenuWidget::view)
        .subscription(MenuWidget::subscription)
        .window(window_settings)
        .style(|_state, _theme| iced::application::Appearance {
            background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.9),
            text_color: Color::WHITE,
        })
        .run()
        //.run_with(|| MenuWidget::new(build_menu()))
}

impl Default for MenuWidget { // TODO maybe can remove due to run_with
    fn default() -> MenuWidget {
        MenuWidget::new(build_menu())
    }
}

// TODO build from config
fn build_menu() -> Vec<MenuEntry> {
    vec!(
        MenuEntry {
            name: "Aaaaa".to_string(),
            shortcut: Some("a".to_string()),
            action: MenuAction::Program { exec: "notify-send aaaaa".to_string() },
        },
        MenuEntry {
            name: "xeyes".to_string(),
            shortcut: Some("x".to_string()),
            action: MenuAction::Program { exec: "xeyes".to_string() },
        },
        MenuEntry {
            name: "Mmmmmmm".to_string(),
            shortcut: Some("m".to_string()),
            action: MenuAction::SubMenu {
                entries: vec!(
                    MenuEntry {
                        name: "Iiiiii".to_string(),
                        shortcut: Some("i".to_string()),
                        action: MenuAction::Program { exec: "notify-send iii".to_string() },
                    },
                    MenuEntry {
                        name: "Hngndgnngn".to_string(),
                        shortcut: None,
                        action: MenuAction::Program { exec: "notify-send Hngndgnngn".to_string() },
                    },
                ),
            }
        }
    )
}
