mod menu;
mod gui;
mod theme;

use menu::{MenuEntry, build_menu_from_file};
use gui::MenuWidget;

use iced::{Color, Theme, theme::Custom};
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
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: "nested-shortcuts".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    iced::application(
        move || {
            let widget = MenuWidget::new(
                menu.clone(),
                std::io::stdout().is_terminal(),
                Some(start)
            );
            eprintln!("Widget created after {:?}", start.elapsed());
            widget
        },
        MenuWidget::update,
        MenuWidget::view,
    )
        .title("nmenu")
        .subscription(MenuWidget::subscription)
        .window(window_settings)
        .theme(app_theme)
        .style(app_style)
        .run()?;
    Ok(())
}

fn app_theme(_state: &MenuWidget) -> Theme {
    Theme::Custom(Custom::new("nested-shortcuts".to_string(), theme::PALETTE_DARK).into())
}

fn app_style(_state: &MenuWidget, _theme: &Theme) -> iced::theme::Style {
    iced::theme::Style {
        background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.95),
        text_color: Color::WHITE,
    }
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
