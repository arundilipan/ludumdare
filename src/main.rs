extern crate ggez;

use ggez::*;
use ggez::graphics::{Point2, Vector2};
use ggez::graphics;
use ggez::timer;

struct Player {
    pos: Point2,
    vel: Vector2,
}

struct InputState {
    vel_x: f64,
    vel_y: f64,

}


impl Default for InputState {
    fn default() -> Self {
        InputState {
            vel_x: 0.0,
            vel_y: 0.0,
        }
    }
}

struct MainState {
    player: Player,
    input_state: InputState,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { 
            player: Player {
                pos: Point2::origin(),
                vel: Vector2::new(0.0, 0.0),
            },

            input_state: InputState::default()
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta = timer::duration_to_f64(timer::get_delta(ctx));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        Ok(())
    }
}

pub fn main() {
    let conf = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("simple", "arundilipan", conf).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
