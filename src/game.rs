use ggez::*;
use ggez::{
    input::keyboard::KeyCode,
    Context, GameResult,
};
use ggez::glam::*;

use crate::structs::State;

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        let delta_seconds = self.dt.as_secs_f32();
        self.playerpos += self.playerspeed * delta_seconds;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.parallax_info.is_splitscreen { 
            self.draw_splitscreen(ctx)
        } else {
            self.draw_parallax(ctx)
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: ggez::input::keyboard::KeyInput, _repeat: bool) -> GameResult {
        if let Some(key) = input.keycode {
            match key {
                KeyCode::Escape | KeyCode::Q => ctx.request_quit(),
                KeyCode::Left => self.playerspeed = -5.0,
                KeyCode::Right => self.playerspeed = 5.0,
                KeyCode::S => self.parallax_info.is_splitscreen = !self.parallax_info.is_splitscreen,
                _ => (),
            }
        }

        //input.keycode.inspect(|x| if *x == KeyCode::Escape {
        //    panic!("thanks for playing")
        //});
        //if input.keycode.is_some_and(|x| x == KeyCode::Escape) {
        //    panic!("Thanks for playing!");
        //}
        //match input.keycode {
        //    Some(KeyCode::Escape) | Some(KeyCode::Q) => panic!("Thanks for playing!"),
        //    _ => (),
        //}
        Ok(())
    }
}