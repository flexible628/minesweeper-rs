use crate::game::{
    components::minefield::{CellState, MineCell},
    GameState,
};

pub enum ButtonAppearance {
    Happy,
    OpenEyed,
    Dead,
    Cool,
    Hovered,
}

pub enum CellAppearance {
    Num(u8),
    Hidden,
    Hovered,
    Mined,
    Flagged,
    Wrong,
    Blown,
}

impl ButtonAppearance {
    pub fn from_gamestate(gamestate: &GameState) -> Self {
        if let &GameState::Finished(won) = gamestate {
            if won {
                Self::Cool
            } else {
                Self::Dead
            }
        } else {
            Self::Happy
        }
    }
}

impl CellAppearance {
    pub fn from_cell_default(cell: &MineCell) -> Self {
        match cell.state() {
            CellState::Hidden => Self::Hidden,
            CellState::Flagged => Self::Flagged,
            CellState::Revealed => match cell.kind().to_int() {
                n @ 0..=8 => Self::Num(n),
                _ => Self::Blown,
            },
        }
    }

    pub fn from_cell_final(cell: &MineCell) -> Option<Self> {
        let appearance = match cell.state() {
            CellState::Hidden => match cell.kind().to_int() {
                n @ 0..=8 => Self::Num(n),
                _ => Self::Mined,
            },
            CellState::Flagged => match cell.kind().to_int() {
                0..=8 => Self::Wrong,
                _ => return None,
            },
            CellState::Revealed => return None,
        };

        Some(appearance)
    }
}
