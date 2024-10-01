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

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) -> GameResult {


        if self.parallax_info.is_splitscreen && ctx.mouse.position().x >= crate::constants::SCREEN_MID_X { 
            let separation_factor: f32 = 1.1;
            if y < 0.0 { // scroll down
                self.adjust_grid_sep_mult(ctx, separation_factor);
            } else if y > 0.0 {  // scroll down
                self.adjust_grid_sep_mult(ctx, 1.0/separation_factor);
            }       
         } else {
            let parallax_mod = 50.0;
            if y < 0.0 { // scroll down
                self.adjust_parallax_linear(ctx, parallax_mod);
            } else if y > 0.0 {  // scroll down
                self.adjust_parallax_linear(ctx, -parallax_mod);
            }
        }

        Ok(())
    }
}