mod game;

use game::{FieldOptions, GameHandler};

use sdl2::{event::Event, mouse::MouseButton, EventPump, Sdl as Context};
use std::time::Instant;

// shortened error types
pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type SdlResult = Result<(), String>;

// start point
pub fn run(cols: usize, rows: usize, mines: usize) -> DynResult<()> {
    //initialization
    let context = sdl2::init()?;
    let options = FieldOptions { cols, rows, mines };

    let mut game_handler = GameHandler::init(&context, options)?;
    let mut event_pump = context.event_pump()?;

    //main game loop
    while game_handler.is_active() {
        handle_game_events(&mut game_handler, &mut event_pump)?;
    }

    Ok(())
}

fn handle_game_events(game_handler: &mut GameHandler, event_pump: &mut EventPump) -> SdlResult {
    let now = Instant::now();

    for event in event_pump.poll_iter() {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => game_handler.left_click(x, y),
            Event::MouseMotion {
                mousestate, x, y, ..
            } => {
                if mousestate.left() {
                    game_handler.mouse_move(x, y);
                }
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => game_handler.left_release(x, y),
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Right,
                x,
                y,
                ..
            } => game_handler.right_click(x, y),
            Event::Quit { .. } => {
                game_handler.quit();
                break;
            }
            _ => (),
        }
    }

    game_handler.next_frame(now)
}
