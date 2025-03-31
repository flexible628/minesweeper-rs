pub mod components;
pub mod rendering;

use components::{
    minefield::{CellState, MineCell},
    GameComponents,
};
use rendering::Renderer;

use super::{Context, DynResult, SdlResult};
use std::time::{Duration, Instant};

pub use components::minefield::FieldOptions;

#[derive(PartialEq)]
pub enum GameState {
    Playing(bool),  // is_idle: bool
    Finished(bool), // won: bool
    Quitted,
}

pub struct GameHandler {
    state: GameState,
    components: GameComponents,
    renderer: Renderer,
}

// public methods
impl GameHandler {
    pub fn init(context: &Context, options: FieldOptions) -> DynResult<Self> {
        let state = GameState::Playing(true);
        let components = GameComponents::new(options);

        let clamped_options = components.minefield.options();
        let renderer = Renderer::init(context, clamped_options)?;

        Ok(Self {
            state,
            components,
            renderer,
        })
    }

    pub fn is_active(&self) -> bool {
        self.state != GameState::Quitted
    }

    pub fn left_click(&mut self, x: i32, y: i32) {
        if self.renderer.button_contains(x, y) {
            self.components.button.click();
            self.renderer.draw_button_hovered();
            return;
        }

        if matches!(self.state, GameState::Playing(_)) {
            self.renderer.draw_button_openeyed();
            self.update_target_cell(x, y);
        }
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        let button = &mut self.components.button;

        if button.is_pressed {
            let hovered = self.renderer.button_contains(x, y);

            if button.is_hovered != hovered {
                button.is_hovered = hovered;
            } else {
                return;
            }

            if hovered {
                self.renderer.draw_button_hovered();
            } else {
                self.renderer.draw_button_by_gamestate(&self.state);
            }

            return;
        }

        if matches!(self.state, GameState::Playing(_)) && self.renderer.is_target_changed(x, y) {
            self.renderer.clear_hovered_cells();
            self.update_target_cell(x, y);
        }
    }

    pub fn left_release(&mut self, x: i32, y: i32) {
        let button = &mut self.components.button;

        if button.is_pressed {
            if button.is_hovered {
                self.restart();
            } else {
                button.is_pressed = false;
                self.renderer.draw_button_by_gamestate(&self.state);
            }

            return;
        }

        if matches!(self.state, GameState::Playing(_)) {
            self.renderer.draw_button_by_gamestate(&self.state);
            self.renderer.clear_hovered_cells();
            self.handle_left_click(x, y);
        }
    }

    pub fn right_click(&mut self, x: i32, y: i32) {
        self.toggle_flag(x, y);
    }

    pub fn quit(&mut self) {
        self.state = GameState::Quitted;
    }

    pub fn next_frame(&mut self, now: Instant) -> SdlResult {
        if let Some(secs) = self.components.secs_counter.get_secs() {
            self.renderer.draw_secs_counter(secs);
        }

        self.renderer.render_frame()?;

        spin_sleep::sleep(Duration::from_nanos(1_000_000_000 / 60).saturating_sub(now.elapsed()));

        Ok(())
    }
}

// private methods
impl GameHandler {
    fn update_target_cell(&mut self, x: i32, y: i32) {
        let cell = match self.get_cell(x, y) {
            Some(cell) => {
                self.renderer.set_target_cell(&cell);
                cell
            }
            None => {
                self.renderer.reset_target_cell();
                return;
            }
        };

        match cell.state() {
            CellState::Hidden => self.renderer.draw_cell_hovered(&cell),
            CellState::Flagged => (),
            CellState::Revealed if cell.kind().to_int() == 0 => (),
            CellState::Revealed => {
                let minefield = &self.components.minefield;
                let (x, y) = (cell.x(), cell.y());

                for (x, y) in minefield.get_coords_around(x, y) {
                    let cell = unsafe { minefield.get_cell_unchecked(x, y) };

                    if cell.is_hidden() {
                        self.renderer.draw_cell_hovered(&cell);
                    }
                }
            }
        }
    }

    fn handle_left_click(&mut self, x: i32, y: i32) {
        let cell = match self.get_cell(x, y) {
            Some(cell) => {
                self.renderer.reset_target_cell();
                cell
            }
            None => return,
        };

        if let GameState::Playing(true) = self.state {
            if cell.is_flagged() {
                return;
            }

            let (init_x, init_y) = (cell.x(), cell.y());
            unsafe { self.components.minefield.place_random_mines(init_x, init_y) };

            self.state = GameState::Playing(false);
            self.components.secs_counter.start();

            self.handle_left_click(x, y);
            return;
        }

        match cell.state() {
            CellState::Hidden => self.open_cell(cell),
            CellState::Flagged => (),
            CellState::Revealed if cell.kind().to_int() == 0 => (),
            CellState::Revealed => {
                let (x, y) = (cell.x(), cell.y());
                let flags_around = self.components.minefield.count_flags_around(x, y);

                if cell.kind().to_int() == flags_around {
                    self.open_around(x, y);
                }
            }
        }

        if self.components.cells_counter.get_count() == 0 {
            self.state = GameState::Finished(true);
        }

        if matches!(self.state, GameState::Finished(_)) {
            self.finish();
        }
    }

    fn open_cell(&mut self, mut cell: MineCell) {
        if cell.is_mined() {
            self.state = GameState::Finished(false);
        } else {
            self.components.cells_counter.decrement();
        }

        cell.set_state(CellState::Revealed);

        self.components.minefield.set_cell(&cell);
        self.renderer.draw_cell_default(&cell);

        if cell.kind().to_int() == 0 {
            self.open_around(cell.x(), cell.y());
        }
    }

    fn open_around(&mut self, x: usize, y: usize) {
        for (x, y) in self.components.minefield.get_coords_around(x, y) {
            let cell = unsafe { self.components.minefield.get_cell_unchecked(x, y) };

            if cell.is_hidden() {
                self.open_cell(cell);
            }
        }
    }

    fn toggle_flag(&mut self, x: i32, y: i32) {
        let mut cell = match self.get_cell(x, y) {
            Some(cell) => cell,
            None => return,
        };

        let flags_counter = &mut self.components.flags_counter;

        match cell.state() {
            CellState::Hidden => {
                flags_counter.decrement();
                cell.set_state(CellState::Flagged);
            }
            CellState::Flagged => {
                flags_counter.increment();
                cell.set_state(CellState::Hidden);
            }
            CellState::Revealed => return,
        }

        self.renderer.draw_flags_counter(flags_counter.get_count());

        self.components.minefield.set_cell(&cell);
        self.renderer.draw_cell_default(&cell);
    }

    fn get_cell(&self, x: i32, y: i32) -> Option<MineCell> {
        if let Some((x, y)) = self.renderer.get_cell_pos(x, y) {
            Some(unsafe { self.components.minefield.get_cell_unchecked(x, y) })
        } else {
            None
        }
    }

    fn restart(&mut self) {
        if let GameState::Playing(true) = self.state {
            self.restart_if_idle();
            return;
        }

        self.state = GameState::Playing(true);
        self.components.reset();

        let flags_count = self.components.flags_counter.get_count();

        self.renderer.draw_initial_state(flags_count);
    }

    fn restart_if_idle(&mut self) {
        let options = self.components.minefield.options();
        let FieldOptions { mines, .. } = options;

        let flags_count = mines as i32;

        self.components.button.release();

        if self.components.flags_counter.get_count() == flags_count {
            self.renderer.draw_button_by_gamestate(&self.state);
            return;
        }

        self.components.flags_counter.set_count(flags_count);
        self.components.minefield.reset();

        self.renderer.draw_initial_state(flags_count);
    }

    fn finish(&mut self) {
        self.components.secs_counter.stop();

        self.renderer.draw_button_by_gamestate(&self.state);

        for (x, y) in self.components.minefield.get_coords_all() {
            let cell = unsafe { self.components.minefield.get_cell_unchecked(x, y) };
            self.renderer.draw_cell_final(&cell);
        }
    }
}
