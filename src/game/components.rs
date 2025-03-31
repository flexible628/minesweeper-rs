pub mod button;
pub mod counters;
pub mod minefield;

use button::Button;
use counters::{Counter, SecsCounter};
use minefield::{FieldOptions, MineField};

pub struct GameComponents {
    pub button: Button,
    pub cells_counter: Counter,
    pub flags_counter: Counter,
    pub secs_counter: SecsCounter,
    pub minefield: MineField,
}

impl GameComponents {
    pub fn new(options: FieldOptions) -> Self {
        let minefield = MineField::new(options);

        let clamped_options = minefield.options();
        let FieldOptions { cols, rows, mines } = clamped_options;

        let cells_count = (cols * rows - mines) as i32;
        let flags_count = mines as i32;

        Self {
            button: Button::default(),
            cells_counter: Counter::new(cells_count),
            flags_counter: Counter::new(flags_count),
            secs_counter: SecsCounter::default(),
            minefield,
        }
    }

    pub fn reset(&mut self) {
        let options = self.minefield.options();
        let FieldOptions { cols, rows, mines } = options;

        let cells_count = (cols * rows - mines) as i32;
        let flags_count = mines as i32;

        self.button.release();
        self.cells_counter.set_count(cells_count);
        self.flags_counter.set_count(flags_count);
        self.secs_counter.stop();
        self.minefield.reset();
    }
}
