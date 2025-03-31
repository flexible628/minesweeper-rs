use super::appearance::{ButtonAppearance, CellAppearance};
use crate::SdlResult;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Texture, WindowCanvas},
    sys::{SDL_SetRenderTarget, SDL_Texture},
};
use std::ptr;

pub fn get_button_src(appearance: ButtonAppearance) -> Rect {
    let x = match appearance {
        ButtonAppearance::Happy => 0,
        ButtonAppearance::OpenEyed => 27,
        ButtonAppearance::Dead => 54,
        ButtonAppearance::Cool => 81,
        ButtonAppearance::Hovered => 108,
    };

    Rect::new(x, 55, 26, 26)
}

pub fn get_appearance_src(appearance: CellAppearance) -> Rect {
    let mut y = 16;

    let x = match appearance {
        CellAppearance::Num(n) => {
            y = 0;
            n as i32 * 16
        }
        CellAppearance::Hidden => 0,
        CellAppearance::Hovered => 16,
        CellAppearance::Mined => 32,
        CellAppearance::Flagged => 48,
        CellAppearance::Wrong => 64,
        CellAppearance::Blown => 80,
    };

    Rect::new(x, y, 16, 16)
}

pub fn get_digit_src(digit: i32) -> Rect {
    Rect::new(12 * digit, 33, 11, 21)
}

pub fn split_flags_by_digits(flags: i32) -> [i32; 3] {
    let flags = flags.clamp(-99, 999);
    let mut digits = [0; 3];

    digits[0] = if flags < 0 { 10 } else { flags / 100 };
    let remaining = flags.abs() % 100;

    digits[1] = remaining / 10;
    digits[2] = remaining % 10;

    digits
}

pub fn split_secs_by_digits(secs: u64) -> [i32; 3] {
    let secs = secs.clamp(0, 999) as i32;
    let mut digits = [0; 3];

    digits[0] = secs / 100;
    digits[1] = (secs % 100) / 10;
    digits[2] = secs % 10;

    digits
}

pub fn get_cell_dst(x: usize, y: usize) -> Rect {
    let x = x as i32 * 16 + 12;
    let y = y as i32 * 16 + 55;

    Rect::new(x, y, 16, 16)
}

pub fn get_cell_pos(x: i32, y: i32) -> (usize, usize) {
    let x = (x as usize - 12) / 16;
    let y = (y as usize - 55) / 16;

    (x, y)
}

pub fn refill_buffer(canvas: &mut WindowCanvas, texture_map: &Texture) -> SdlResult {
    let viewport = canvas.viewport();
    let field_width = viewport.width() as i32 - 24;
    let field_height = viewport.height() as i32 - 67;

    // drawing top panel
    // let src = Rect::new(70, 82, 1, 1);
    // let dst = Rect::new(12, 11, 1, 1);

    // for x in 0..minefield_width {
    //     let dst = dst.right_shifted(x);

    //     for y in 0..33 {
    //         canvas.copy(texture_map, src, dst.bottom_shifted(y))?;
    //     }
    // }
    // using hardcoded color instead
    // since the skinchange ability hasn't been implemented yet
    canvas.set_draw_color(Color::RGB(192, 192, 192));
    canvas.clear();

    // setting offset for mirrored images (left to right)
    let src_offset = 15;
    let dst_offset = 12 + field_width;

    // drawing three left corners
    let src_top = Rect::new(0, 82, 12, 11);
    let dst_top = Rect::new(0, 0, 12, 11);

    let src_mid = Rect::new(0, 96, 12, 11);
    let dst_mid = Rect::new(0, 44, 12, 11);

    let src_low = Rect::new(0, 110, 12, 12);
    let dst_low = Rect::new(0, 55 + field_height, 12, 12);

    canvas.copy(texture_map, src_top, dst_top)?;
    canvas.copy(texture_map, src_mid, dst_mid)?;
    canvas.copy(texture_map, src_low, dst_low)?;

    // drawing three right corners
    let src_top = src_top.right_shifted(src_offset);
    let dst_top = dst_top.right_shifted(dst_offset);

    let src_mid = src_mid.right_shifted(src_offset);
    let dst_mid = dst_mid.right_shifted(dst_offset);

    let src_low = src_low.right_shifted(src_offset);
    let dst_low = dst_low.right_shifted(dst_offset);

    canvas.copy(texture_map, src_top, dst_top)?;
    canvas.copy(texture_map, src_mid, dst_mid)?;
    canvas.copy(texture_map, src_low, dst_low)?;

    // drawing three horizontal borders
    let src_top = Rect::new(13, 82, 1, 11);
    let dst_top = Rect::new(12, 0, 1, 11);

    let src_mid = Rect::new(13, 96, 1, 11);
    let dst_mid = Rect::new(12, 44, 1, 11);

    let src_low = Rect::new(13, 110, 1, 12);
    let dst_low = Rect::new(12, 55 + field_height, 1, 12);

    for x in 0..field_width {
        canvas.copy(texture_map, src_top, dst_top.right_shifted(x))?;
        canvas.copy(texture_map, src_mid, dst_mid.right_shifted(x))?;
        canvas.copy(texture_map, src_low, dst_low.right_shifted(x))?;
    }

    // drawing two upper vertical borders
    let src_left = Rect::new(0, 94, 12, 1);
    let dst_left = Rect::new(0, 11, 12, 1);

    let src_right = src_left.right_shifted(src_offset);
    let dst_right = dst_left.right_shifted(dst_offset);

    for y in 0..33 {
        canvas.copy(texture_map, src_left, dst_left.bottom_shifted(y))?;
        canvas.copy(texture_map, src_right, dst_right.bottom_shifted(y))?;
    }

    // drawing two bottom vertical borders
    let src_left = Rect::new(0, 108, 12, 1);
    let dst_left = Rect::new(0, 55, 12, 1);

    let src_right = src_left.right_shifted(src_offset);
    let dst_right = dst_left.right_shifted(dst_offset);

    for y in 0..field_height {
        canvas.copy(texture_map, src_left, dst_left.bottom_shifted(y))?;
        canvas.copy(texture_map, src_right, dst_right.bottom_shifted(y))?;
    }

    // drawing two counters
    let src = Rect::new(28, 82, 41, 25);
    let dst = Rect::new(16, 15, 41, 25);

    canvas.copy(texture_map, src, dst)?;
    canvas.copy(texture_map, src, dst.right_shifted(field_width - 49))?;

    Ok(())
}

pub trait WindowCanvasExtension {
    fn set_render_target_raw(&self, texture_raw: *mut SDL_Texture) -> SdlResult;
    fn set_render_target(&self, texture: &Texture) -> SdlResult;
    fn reset_render_target(&self) -> SdlResult;
}

impl WindowCanvasExtension for WindowCanvas {
    fn set_render_target_raw(&self, texture_raw: *mut SDL_Texture) -> SdlResult {
        if unsafe { SDL_SetRenderTarget(self.raw(), texture_raw) } == 0 {
            Ok(())
        } else {
            Err(sdl2::get_error())
        }
    }

    fn set_render_target(&self, texture: &Texture) -> SdlResult {
        self.set_render_target_raw(texture.raw())
    }

    fn reset_render_target(&self) -> SdlResult {
        self.set_render_target_raw(ptr::null_mut())
    }
}
