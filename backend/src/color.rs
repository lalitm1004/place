// Placeholder for now

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Cyan,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    Lime,
    Pink,
    Navy,
}

impl Color {
    pub fn get_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::White => (255, 255, 255),
            Color::Black => (0, 0, 0),
            Color::Red => (255, 0, 0),
            Color::Green => (0, 255, 0),
            Color::Blue => (0, 0, 255),
            Color::Yellow => (255, 255, 0),
            Color::Orange => (255, 165, 0),
            Color::Purple => (128, 0, 128),
            Color::Cyan => (0, 255, 255),
            Color::Magenta => (255, 0, 255),
            Color::Brown => (139, 69, 19),
            Color::LightGray => (211, 211, 211),
            Color::DarkGray => (105, 105, 105),
            Color::Lime => (50, 205, 50),
            Color::Pink => (255, 192, 203),
            Color::Navy => (0, 0, 128),
        }
    }

    pub fn get_hex(&self) -> &'static str {
        match self {
            Color::White => "#FFFFFF",
            Color::Black => "#000000",
            Color::Red => "#FF0000",
            Color::Green => "#00FF00",
            Color::Blue => "#0000FF",
            Color::Yellow => "#FFFF00",
            Color::Orange => "#FFA500",
            Color::Purple => "#800080",
            Color::Cyan => "#00FFFF",
            Color::Magenta => "#FF00FF",
            Color::Brown => "#8B4513",
            Color::LightGray => "#D3D3D3",
            Color::DarkGray => "#696969",
            Color::Lime => "#32CD32",
            Color::Pink => "#FFC0CB",
            Color::Navy => "#000080",
        }
    }
}
