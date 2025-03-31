#[derive(Default)]
pub struct Button {
    pub is_pressed: bool,
    pub is_hovered: bool,
}

impl Button {
    pub fn click(&mut self) {
        self.is_pressed = true;
        self.is_hovered = true;
    }

    pub fn release(&mut self) {
        self.is_pressed = false;
        self.is_hovered = false;
    }
}
