mod menu;
mod gui;
mod theme;

use menu::{MenuEntry, build_menu_from_file};
use gui::MenuWidget;

use iced::{Color, Task, Theme, theme::Custom};
use std::{
    env,
    io::IsTerminal,
    time::Instant,
};

fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let menu = build_menu_from_args()?;
    eprintln!("Menu read after {:?}", start.elapsed());

    let window_settings = iced::window::Settings {
        resizable: false,
        decorations: false,
        transparent: true,
        exit_on_close_request: true,
        position: iced::window::Position::Centered,
        level: iced::window::Level::AlwaysOnTop,
        ..Default::default()
    };
    iced::application("nmenu", MenuWidget::update, MenuWidget::view)
        .subscription(MenuWidget::subscription)
        .window(window_settings)
        .theme(move |_state| Theme::Custom(Custom::new("nested-shortcuts".to_string(), theme::PALETTE_DARK).into()))
        .style(|_state, _theme| iced::theme::Style {
            background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.95),
            text_color: Color::WHITE,
        })
        .run_with(move || {
            let widget = MenuWidget::new(
                menu,
                std::io::stdout().is_terminal(),
                Some(start)
            );
            eprintln!("Widget created after {:?}", start.elapsed());
            (widget.into(), Task::none())
        })?;
    Ok(())
}

/*
impl Default for MenuWidget {
    fn default() -> MenuWidget {
        MenuWidget::new(
            build_menu_from_args().expect("Failed to read menu"),
            std::io::stdout().is_terminal(),
        )
    }
}
*/

fn build_menu_from_args() -> anyhow::Result<Vec<MenuEntry>> {
    let args: Vec<String> = env::args().collect();
    let mut result = Vec::new();
    for path in &args[1..] {
        let mut add = build_menu_from_file(path)?;
        result.append(&mut add);
    }
    if result.len() == 0 {
        Err(anyhow::anyhow!("Empty menu, forgot to provide valid path to a menu.yml?"))
    } else {
        Ok(result)
    }
}
