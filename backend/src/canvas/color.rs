#[derive(Debug, Clone, Copy)]
pub enum Color {
    Maroon,      // #6d001a
    Crimson,     // #be0039
    OrangeRed,   // #ff4500
    Amber,       // #ffa800
    Gold,        // #ffd635
    LightYellow, // #fff8b8
    ForestGreen, // #00a368
    Emerald,     // #00cc78
    LightGreen,  // #7eed56
    Teal,        // #00756f
    SkyBlue,     // #009eaa
    Aqua,        // #00ccc0
    RoyalBlue,   // #2450a4
    DodgerBlue,  // #3690ea
    CyanLight,   // #51e9f4
    Indigo,      // #493ac1
    BlueViolet,  // #6a5cff
    LightBlue,   // #94b3ff
    DarkMagenta, // #811e9f
    Orchid,      // #b44ac0
    Lavender,    // #e4abff
    DeepPink,    // #de107f
    HotPink,     // #ff3881
    LightPink,   // #ff99aa
    SaddleBrown, // #6d482f
    Sienna,      // #9c6926
    Peach,       // #ffb470
    Black,       // #000000
    DarkGray,    // #515252
    Gray,        // #898d90
    LightGray,   // #d4d7d9
    White,       // #ffffff
}

impl Color {
    pub fn get_hex(&self) -> &'static str {
        match self {
            Color::Maroon => "#6d001a",
            Color::Crimson => "#be0039",
            Color::OrangeRed => "#ff4500",
            Color::Amber => "#ffa800",
            Color::Gold => "#ffd635",
            Color::LightYellow => "#fff8b8",
            Color::ForestGreen => "#00a368",
            Color::Emerald => "#00cc78",
            Color::LightGreen => "#7eed56",
            Color::Teal => "#00756f",
            Color::SkyBlue => "#009eaa",
            Color::Aqua => "#00ccc0",
            Color::RoyalBlue => "#2450a4",
            Color::DodgerBlue => "#3690ea",
            Color::CyanLight => "#51e9f4",
            Color::Indigo => "#493ac1",
            Color::BlueViolet => "#6a5cff",
            Color::LightBlue => "#94b3ff",
            Color::DarkMagenta => "#811e9f",
            Color::Orchid => "#b44ac0",
            Color::Lavender => "#e4abff",
            Color::DeepPink => "#de107f",
            Color::HotPink => "#ff3881",
            Color::LightPink => "#ff99aa",
            Color::SaddleBrown => "#6d482f",
            Color::Sienna => "#9c6926",
            Color::Peach => "#ffb470",
            Color::Black => "#000000",
            Color::DarkGray => "#515252",
            Color::Gray => "#898d90",
            Color::LightGray => "#d4d7d9",
            Color::White => "#ffffff",
        }
    }
}
