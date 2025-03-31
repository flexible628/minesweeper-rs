#![windows_subsystem = "windows"]

fn main() -> minesweeper::DynResult<()> {
    let cols = 16;
    let rows = 16;
    let mines = 40;

    minesweeper::run(cols, rows, mines)
}
