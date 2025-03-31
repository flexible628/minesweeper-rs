mod appearance;
mod layout;
mod texture;
mod utils;

use appearance::{ButtonAppearance, CellAppearance};
use layout::Layout;
use texture::TEXTURE_BYTES;
use utils::WindowCanvasExtension;

use crate::{
    game::{
        components::minefield::{FieldOptions, MineCell},
        GameState,
    },
    Context, DynResult, SdlResult,
};
use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Texture, WindowCanvas},
};
use std::mem;

pub struct Renderer {
    canvas: WindowCanvas,
    render_buffer: Texture,
    texture_map: Texture,
    layout: Layout,
    copy_error: String,
    redraw_needed: bool,
}

// public methods (general)
impl Renderer {
    pub fn init(context: &Context, options: FieldOptions) -> DynResult<Self> {
        let title = "Minesweeper";
        let width = options.cols as u32 * 16 + 24;
        let height = options.rows as u32 * 16 + 67;
        let flags_count = options.mines as i32;

        let canvas = context
            .video()?
            .window(title, width, height)
            .position_centered()
            .build()?
            .into_canvas()
            .software()
            .target_texture()
            .build()?;

        let texture_creator = canvas.texture_creator();
        let render_buffer = texture_creator.create_texture_target(None, width, height)?;
        canvas.set_render_target(&render_buffer)?;

        let mut renderer = Self {
            canvas,
            render_buffer,
            texture_map: texture_creator.load_texture_bytes(TEXTURE_BYTES)?,
            layout: Layout::new(width, height),
            copy_error: String::new(),
            redraw_needed: false,
        };

        utils::refill_buffer(&mut renderer.canvas, &renderer.texture_map)?;
        renderer.draw_initial_state(flags_count);

        Ok(renderer)
    }

    pub fn render_frame(&mut self) -> SdlResult {
        if self.redraw_needed {
            if !self.copy_error.is_empty() {
                return Err(mem::take(&mut self.copy_error));
            }

            self.canvas.reset_render_target()?;
            self.canvas.clear();
            self.canvas.copy(&self.render_buffer, None, None)?;
            self.canvas.present();
            self.canvas.set_render_target(&self.render_buffer)?;

            self.redraw_needed = false;
        }

        Ok(())
    }
}

// public methods (drawings)
impl Renderer {
    pub fn draw_initial_state(&mut self, flags_count: i32) {
        self.draw_button(ButtonAppearance::Happy);
        self.draw_flags_counter(flags_count);
        self.draw_secs_counter(0);
        self.draw_blank_minefield();
    }

    pub fn draw_button_by_gamestate(&mut self, gamestate: &GameState) {
        self.draw_button(ButtonAppearance::from_gamestate(gamestate));
    }

    pub fn draw_button_openeyed(&mut self) {
        self.draw_button(ButtonAppearance::OpenEyed);
    }

    pub fn draw_button_hovered(&mut self) {
        self.draw_button(ButtonAppearance::Hovered);
    }

    pub fn draw_flags_counter(&mut self, flags: i32) {
        let digits = utils::split_flags_by_digits(flags);
        let dst = self.layout.flags_digit1_pos;

        self.draw_counter(digits, dst);
    }

    pub fn draw_secs_counter(&mut self, secs: u64) {
        let digits = utils::split_secs_by_digits(secs);
        let dst = self.layout.secs_digit1_pos;

        self.draw_counter(digits, dst);
    }

    pub fn draw_cell_default(&mut self, cell: &MineCell) {
        let appearance = CellAppearance::from_cell_default(cell);

        self.draw_cell(appearance, cell.x(), cell.y());
    }

    pub fn draw_cell_final(&mut self, cell: &MineCell) {
        if let Some(appearance) = CellAppearance::from_cell_final(cell) {
            self.draw_cell(appearance, cell.x(), cell.y());
        }
    }

    pub fn draw_cell_hovered(&mut self, cell: &MineCell) {
        let (x, y) = (cell.x(), cell.y());

        self.layout.hovered_cells.push((x, y));
        self.draw_cell(CellAppearance::Hovered, x, y);
    }

    pub fn clear_hovered_cells(&mut self) {
        while let Some((x, y)) = self.layout.hovered_cells.pop() {
            self.draw_cell(CellAppearance::Hidden, x, y);
        }
    }
}

// public methods (layout)
impl Renderer {
    pub fn button_contains(&self, x: i32, y: i32) -> bool {
        self.layout.button_pos.contains_point((x, y))
    }

    pub fn minefield_contains(&self, x: i32, y: i32) -> bool {
        self.layout.minefield_pos.contains_point((x, y))
    }

    pub fn get_cell_pos(&self, x: i32, y: i32) -> Option<(usize, usize)> {
        if self.minefield_contains(x, y) {
            Some(utils::get_cell_pos(x, y))
        } else {
            None
        }
    }

    pub fn is_target_changed(&self, x: i32, y: i32) -> bool {
        match self.layout.target_pos {
            Some(dst) => !dst.contains_point((x, y)),
            None => self.minefield_contains(x, y),
        }
    }

    pub fn set_target_cell(&mut self, cell: &MineCell) {
        let dst = utils::get_cell_dst(cell.x(), cell.y());
        self.layout.target_pos = Some(dst);
    }

    pub fn reset_target_cell(&mut self) {
        self.layout.target_pos = None;
    }
}

// private methods (drawings)
impl Renderer {
    fn draw_button(&mut self, appearance: ButtonAppearance) {
        let src = utils::get_button_src(appearance);
        let dst = self.layout.button_pos;

        self.draw_part(src, dst);
        self.redraw_needed = true;
    }

    fn draw_counter(&mut self, digits: [i32; 3], mut dst: Rect) {
        for digit in digits {
            self.draw_part(utils::get_digit_src(digit), dst);
            dst.set_x(dst.x + 13);
        }

        self.redraw_needed = true;
    }

    fn draw_cell(&mut self, appearance: CellAppearance, x: usize, y: usize) {
        let src = utils::get_appearance_src(appearance);
        let dst = utils::get_cell_dst(x, y);

        self.draw_part(src, dst);
        self.redraw_needed = true;
    }

    fn draw_blank_minefield(&mut self) {
        let minefield_pos = self.layout.minefield_pos;
        let field_width = minefield_pos.width() as i32;
        let field_height = minefield_pos.height() as i32;

        let src = utils::get_appearance_src(CellAppearance::Hidden);
        let dst = utils::get_cell_dst(0, 0);

        for x in (0..field_width).step_by(16) {
            let dst = dst.right_shifted(x);

            for y in (0..field_height).step_by(16) {
                self.draw_part(src, dst.bottom_shifted(y));
            }
        }

        self.redraw_needed = true;
    }

    fn draw_part(&mut self, src: Rect, dst: Rect) {
        let result = self.canvas.copy(&self.texture_map, src, dst);

        if let Err(error) = result {
            self.copy_error = error;
        };
    }
}
