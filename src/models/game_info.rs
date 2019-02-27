#[derive(PartialEq, Clone, Copy)]
pub enum GameMode {
    PlayerVsPlayer,
    PlayerVsIa,
    IaVsIa
}

pub struct GameInfo {
    pub mode: GameMode,
    pub first_ia_depth: u8,
    pub second_ia_depth: u8,
    pub display_weight: bool,
}

impl GameInfo {
    pub fn new() -> GameInfo {
        GameInfo {
            mode: GameMode::PlayerVsPlayer,
            first_ia_depth: 0 as u8,
            second_ia_depth: 0 as u8,
            display_weight: false
        }
    }
}
