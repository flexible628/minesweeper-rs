use sdl2::rect::Rect;

pub struct Layout {
    pub button_pos: Rect,
    pub flags_digit1_pos: Rect,
    pub secs_digit1_pos: Rect,
    pub minefield_pos: Rect,
    pub target_pos: Option<Rect>,
    pub hovered_cells: Vec<(usize, usize)>,
}

impl Layout {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            button_pos: Rect::from_center((width as i32 / 2, 28), 26, 26),
            flags_digit1_pos: Rect::new(18, 17, 11, 21),
            secs_digit1_pos: Rect::new(width as i32 - 55, 17, 11, 21),
            minefield_pos: Rect::new(12, 55, width - 24, height - 67),
            target_pos: None,
            hovered_cells: Vec::with_capacity(8),
        }
    }
}
