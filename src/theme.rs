use iced::{
    Color,
    theme::Palette,
};

pub static PALETTE_DARK: Palette = Palette {
    background: Color::TRANSPARENT,
    text: Color::WHITE,
    primary: Color::TRANSPARENT,
    success: Color::from_rgba(0.0, 1.0, 0.0, 1.0),
    danger: Color::from_rgba(1.0, 0.0, 0.0, 1.0),
    warning: Color::from_rgba(1.0, 0.0, 0.0, 1.0),
};
