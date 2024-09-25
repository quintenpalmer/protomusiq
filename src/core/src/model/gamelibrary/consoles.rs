#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GameConsole {
    GameBoy,
    GameBoyColor,
    GameBoyAdvance,
    NintendoDS,
    SNES,
    Nintendo64,
    GameCube,
    Wii,
}

impl GameConsole {
    pub fn full_name(&self) -> String {
        match self {
            GameConsole::GameBoy => "Game Boy",
            GameConsole::GameBoyColor => "Game Boy Color",
            GameConsole::GameBoyAdvance => "Game Boy Advance",
            GameConsole::NintendoDS => "Nintendo DS",
            GameConsole::SNES => "Super Nintendo Entertainment System",
            GameConsole::Nintendo64 => "Nintendo 64",
            GameConsole::GameCube => "GameCube",
            GameConsole::Wii => "Wii",
        }
        .to_string()
    }

    pub fn from_full_name(s: String) -> Option<Self> {
        match s.as_str() {
            "Game Boy" => Some(GameConsole::GameBoy),
            "Game Boy Color" => Some(GameConsole::GameBoyColor),
            "Game Boy Advance" => Some(GameConsole::GameBoyAdvance),
            "Nintendo DS" => Some(GameConsole::NintendoDS),
            "Super Nintendo Entertainment System" => Some(GameConsole::SNES),
            "Nintendo 64" => Some(GameConsole::Nintendo64),
            "GameCube" => Some(GameConsole::GameCube),
            "Wii" => Some(GameConsole::Wii),
            _ => None,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            GameConsole::GameBoy,
            GameConsole::GameBoyColor,
            GameConsole::GameBoyAdvance,
            GameConsole::NintendoDS,
            GameConsole::SNES,
            GameConsole::Nintendo64,
            GameConsole::GameCube,
            GameConsole::Wii,
        ]
    }
}
