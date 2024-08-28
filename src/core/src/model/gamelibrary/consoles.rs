#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GameConsole {
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

    pub fn all() -> Vec<Self> {
        vec![
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
