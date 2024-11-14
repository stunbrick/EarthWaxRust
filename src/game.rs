use ggez::*;
use ggez::{
    input::keyboard::KeyCode,
    Context, GameResult,
};
use ggez::glam::*;

use std::rc::Rc;
use crate::{
    State, UnitType, WorldPos, Unit, AnimatedSpriteInfo, AnimatedRenderable, Spritesheet, UnitState, Anim, RabbitAnim,
    MovementSystem,
};

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let (grub_sprite, grub_sprite_info) = self.animation_system.get_sprite_and_info_for_unit(UnitType::Grubling);
        if ! self.done_once {
            self.done_once = true;

            let new_grubs = spawn_grid_of_units(
                UnitType::Grubling,
                &grub_sprite,
                grub_sprite_info,
                20,
                4,
                -20,
            );
            self.units.extend(new_grubs);

            let (rabbit_sprite, rabbit_sprite_info) = self.animation_system.get_sprite_and_info_for_unit(UnitType::Rabbit);
            let mut new_rabbits = spawn_grid_of_units(
                UnitType::Rabbit,
                &rabbit_sprite,
                rabbit_sprite_info,
                20,
                4,
                0,
            );
            let mut i = 0;
            for mut rabbit in &mut new_rabbits {
                i += 1;
                if i%2 == 0 {

                    rabbit.state = UnitState::Move;
                    MovementSystem::order_march_to(&mut rabbit, WorldPos::new(0.0, -50.0, 0.0));
                    self.animation_system.change_unit_anim(
                        &mut rabbit,
                        Anim::Rabbit(RabbitAnim::Run)
                    );
                    
                }
            }


            self.units.extend(new_rabbits);
        }
        let rabbits: Vec<&mut Unit> = self.units.iter_mut()
            .filter(|unit| matches!(unit.unit_type, UnitType::Rabbit))
            .collect();
        for mut rabbit in &mut self.units {
            if rabbit.state == UnitState::Idle {
                
            }
        }
        self.dt = ctx.time.delta();
        let delta_seconds = self.dt.as_secs_f32();
        self.playerpos += self.playerspeed * delta_seconds;

        self.animation_system.animate_units(&mut self.units, delta_seconds);
        MovementSystem::move_any_moving(&mut self.units, delta_seconds, &self.animation_system);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
    // TODO This is just for animation testing 

        // if self.is_drawing_gremlin {
        //     return self.draw_gremlin(ctx)
        // }
        
        return self.draw_with_lawn(ctx);
        // if self.is_batching { 
        //     self.draw_parallax_batched(ctx)
        // } else if self.parallax_info.is_splitscreen { 
        //     self.draw_splitscreen(ctx)
        // } else {
        //     // self.draw_parallax(ctx)
        //     self.draw_gremlin(ctx)
        // }
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: ggez::input::keyboard::KeyInput, _repeat: bool) -> GameResult {
        if let Some(key) = input.keycode {
            match key {
                KeyCode::Escape | KeyCode::Q => ctx.request_quit(),
                KeyCode::Left => self.playerspeed = -5.0,
                KeyCode::Right => self.playerspeed = 5.0,
                KeyCode::Down => self.playerspeed = 0.0,
                KeyCode::S => self.parallax_info.is_splitscreen = !self.parallax_info.is_splitscreen,
                // Cycle the color. This is a bad function and should be removed.
                KeyCode::B => {
                    self.is_batching = !self.is_batching;
                    let is_batching = self.is_batching;
                    println!("Batching? {is_batching}");
                     // See the color change when we change batch mode lol
                    // if is_batching { 
                    //     self.parallax_info.background_color_index = 1
                    // } else  {
                    //     self.parallax_info.background_color_index = 2
                    // }
                }
                KeyCode::C => self.parallax_info.background_color_index = new_color_index(self.parallax_info.background_color_index),
                KeyCode::V => {
                    self.playerspeed = -25.0;
                    self.parallax_info.background_color_index = new_color_index(self.parallax_info.background_color_index);
                }
                // KeyCode::F => {
                //     let x: &mut u32 = &mut self.gremlin_sprite_sheet.frame;
                //     *x = (*x + 1) % 6;
                // }
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

fn spawn_unit(
    unit_type: UnitType,
    sprite: &Rc<ggez::graphics::Image>,
    sprite_info: AnimatedSpriteInfo,
    world_pos: WorldPos,
    ) -> Unit {
    Unit {
        unit_type,
        animated_renderable: AnimatedRenderable {
            sprite: Spritesheet {
                image: sprite.clone(),
                frame: sprite_info.frame,               // which frame you are on
                sprite_width: sprite_info.sprite_width, // width of a single frame
                sprite_height: sprite_info.sprite_height, // height of a single frame
                hor_frames: sprite_info.hor_frames,     // how many frames horizontally
                total_frames: sprite_info.total_frames,
            },
            anim_time: sprite_info.frame as f32,
            anim_speed: 6.0, // how many frames a second to animate
            flip_x: false,
        },
        world_pos,
        destination: world_pos,
        state: UnitState::Idle
    }
}

fn spawn_units(
    unit_type: UnitType,
    sprite: &std::rc::Rc<graphics::Image>,
    sprite_info: AnimatedSpriteInfo,
    unit_positions: Vec<WorldPos>,
) -> Vec<Unit> {
    let mut units: Vec<Unit> = Vec::new();
    for unit_pos in unit_positions.into_iter() {
        let new_frame = ((unit_pos.x.abs() as u32) + unit_pos.depth as u32) % 6;
        let new_sprite_info = AnimatedSpriteInfo {
            frame: new_frame,
            sprite_width: sprite_info.sprite_width,
            sprite_height: sprite_info.sprite_height,
            hor_frames: sprite_info.hor_frames,
            total_frames: sprite_info.total_frames,
        };
        let unit = spawn_unit(unit_type, sprite, new_sprite_info, unit_pos);
        units.push(unit);
    }
    units
}

fn spawn_grid_of_units(
    unit_type: UnitType,
    sprite: &std::rc::Rc<graphics::Image>,
    sprite_info: AnimatedSpriteInfo,
    x: i32,
    depth: i32,
    offset_x: i32,
) -> Vec<Unit> {
    let mut unit_positions: Vec<WorldPos> = Vec::new();
    for x in 0 + offset_x..x + offset_x {
        for depth in 1..depth {
            let world_pos = WorldPos {
                x: (x * 4) as f32,
                height: 0.0,
                depth: (depth * 4) as f32,
            };
            unit_positions.push(world_pos);
        }
    }
    let units = spawn_units(unit_type, &sprite, sprite_info, unit_positions);
    units
}

// This is a bad function and should be removed
fn new_color_index(old_color_index : u8) -> u8 { 
    if old_color_index < 3 { 
        old_color_index+1
    } else { 
        1
    }
}

