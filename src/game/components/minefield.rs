mod minecell;

use minecell::MineCell as CellInternal;

pub use minecell::{CellKind, CellState};

pub struct MineCell {
    cell: CellInternal,
    x: usize,
    y: usize,
}

pub struct MineField {
    options: FieldOptions,
    cells: Vec<Vec<CellInternal>>,
}

impl MineCell {
    pub fn kind(&self) -> CellKind {
        self.cell.kind
    }

    pub fn state(&self) -> CellState {
        self.cell.state
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn is_mined(&self) -> bool {
        self.cell.kind == CellKind::Mined
    }

    pub fn is_hidden(&self) -> bool {
        self.cell.state == CellState::Hidden
    }

    pub fn is_flagged(&self) -> bool {
        self.cell.state == CellState::Flagged
    }

    // (unused)
    // pub fn is_revealed(&self) -> bool {
    //     self.cell.state == CellState::Revealed
    // }

    pub fn set_state(&mut self, state: CellState) {
        self.cell.state = state;
    }
}

impl MineField {
    pub fn new(options: FieldOptions) -> Self {
        let cols = options.cols.clamp(9, 30);
        let rows = options.rows.clamp(9, 30);
        let mines = options.mines.clamp(10, cols * rows - 1);

        let options = FieldOptions { cols, rows, mines };
        let cells = vec![vec![CellInternal::default(); rows]; cols];

        Self { options, cells }
    }

    pub fn options(&self) -> FieldOptions {
        self.options
    }

    pub unsafe fn get_cell_unchecked(&self, x: usize, y: usize) -> MineCell {
        let cell = self.cells[x][y];

        MineCell { cell, x, y }
    }

    pub fn get_coords_all(&self) -> impl Iterator<Item = (usize, usize)> {
        let x_range = 0..self.options.cols;
        let y_range = 0..self.options.rows;

        x_range.flat_map(move |x| y_range.clone().map(move |y| (x, y)))
    }

    pub fn get_coords_around(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let x_range = x.saturating_sub(1)..(x + 2).min(self.options.cols);
        let y_range = y.saturating_sub(1)..(y + 2).min(self.options.rows);

        x_range.flat_map(move |x| y_range.clone().map(move |y| (x, y)))
    }

    pub fn count_flags_around(&self, x: usize, y: usize) -> u8 {
        self.get_coords_around(x, y)
            .filter(|&(x, y)| self.cells[x][y].state == CellState::Flagged)
            .count() as u8
    }

    pub fn set_cell(&mut self, cell: &MineCell) {
        self.cells[cell.x][cell.y].state = cell.cell.state;
    }

    pub fn reset(&mut self) {
        self.cells
            .iter_mut()
            .flat_map(|col| col.iter_mut())
            .for_each(|cell| *cell = CellInternal::default());
    }

    pub unsafe fn place_random_mines(&mut self, init_x: usize, init_y: usize) {
        use rand::{rng, seq::index::sample};

        let FieldOptions { cols, rows, mines } = self.options;

        let init_index = init_x + init_y * cols;

        sample(&mut rng(), cols * rows - 1, mines)
            .into_iter()
            .for_each(|mut index| {
                if index >= init_index {
                    index += 1;
                }

                let x = index % cols;
                let y = index / cols;

                self.cells[x][y].kind = CellKind::Mined;

                for (x, y) in self.get_coords_around(x, y) {
                    self.cells[x][y].kind.increment()
                }
            });
    }
}

#[derive(Clone, Copy)]
pub struct FieldOptions {
    pub cols: usize,
    pub rows: usize,
    pub mines: usize,
}
