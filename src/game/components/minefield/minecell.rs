#[derive(Default, Clone, Copy)]
pub struct MineCell {
    pub kind: CellKind,
    pub state: CellState,
}

#[repr(u8)]
#[derive(Default, Clone, Copy, PartialEq)]
pub enum CellKind {
    #[default]
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Mined,
}

impl CellKind {
    pub fn to_int(self) -> u8 {
        self as u8
    }

    pub fn increment(&mut self) {
        *self = match self {
            Self::Num0 => Self::Num1,
            Self::Num1 => Self::Num2,
            Self::Num2 => Self::Num3,
            Self::Num3 => Self::Num4,
            Self::Num4 => Self::Num5,
            Self::Num5 => Self::Num6,
            Self::Num6 => Self::Num7,
            Self::Num7 => Self::Num8,
            Self::Num8 => Self::Num8,
            Self::Mined => Self::Mined,
        };
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum CellState {
    #[default]
    Hidden,
    Flagged,
    Revealed,
}
