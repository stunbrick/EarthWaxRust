mod structs;
mod constants;
mod game;
mod parallax;

use structs::*;
use crate::parallax::*;

use ggez::*;
use std::rc::Rc;
use ggez::{
    event,
    glam::*,
};
use std::collections::BTreeMap;


pub fn main() {
    let resource_dir: std::path::PathBuf = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        std::path::PathBuf::from("./assets")
    };

    let mut c = conf::Conf::new();
    c.window_mode.width = 1920.0;
    c.window_mode.height = 1080.0;
    c.window_setup = ggez::conf::WindowSetup {
        title: "Earthwax Impetus".to_owned(),
        samples: ggez::conf::NumSamples::One,
        vsync: true,
        icon: "".to_owned(), // don't know how to get it to find the icon,
        srgb: true,
    };
    let (ctx, event_loop) = ContextBuilder::new("earthwax", "broskisChimes")
        .default_conf(c)
        .add_resource_path(resource_dir)
        .build()
        .expect("Holy fuck I lost all context");

    //let mut chickens = Vec::new();
    //let chicken_sprite = ggez::graphics::Image::from_path(&ctx, "/chicken_idle.png").expect("Holy fuck no chicken_sprite!");
    //let chicken_sprite_clone = Rc::new(chicken_sprite);
    //for i in -10..10 {
    //    for j in 0..4 {
    //        let chicken = Renderable {
    //            sprite: Rc::clone(&chicken_sprite_clone),
    //            world_pos: WorldPos {
    //                x: (i * 2) as f32,
    //                height: 0.0,
    //                depth: (j * 2) as f32,
    //            }
    //        };
    //        chickens.push(chicken);
    //    }
    //}
    let mut men: Vec<Renderable> = Vec::new();

    let man_sprite = 
        ggez::graphics::Image::from_path(&ctx, "/farmer_idle.png")
        .expect("Holy fuck no man_sprite!");

    let man_sprite_clone = Rc::new(man_sprite);
    let mut men_positions: Vec<WorldPos> = Vec::new();
    for i in -2..=2 {
        for j in 1..=4 {
            let world_pos = WorldPos {
                x: (i * 4) as f32,
                height: 0.0,
                depth: (j * 4) as f32,
            };
            men_positions.push(world_pos);
        }
    }
    let men = spawn_men(&man_sprite_clone, men_positions);


    let man_sprite_for_batch_test = 
        ggez::graphics::Image::from_path(&ctx, "/farmer_idle.png")
        .expect("Holy Fuck, Batchman!");
    // let sprite_batch =  ggez::graphics::InstanceArray::new_ordered(&ctx, man_sprite_for_batch);

    let grass_sprite = 
        ggez::graphics::Image::from_path(&ctx, "/grass_small.png")
        .expect("Who smoked all the grass?!");

    let grubling_sprite_sheet_image = 
        ggez::graphics::Image::from_path(&ctx, "/grub_small_attack.png")
        .expect("Don't feed the grublings after midnight!");
    let grubling_sprite_clone: Rc<graphics::Image> = Rc::new(grubling_sprite_sheet_image);

    let mut animated_renderables: Vec<AnimatedRenderable> =  Vec::new();

    let mut grublings = spawn_grid_of_units(&grubling_sprite_clone, 20, 4, -20);

    //let mut grubling_positions: Vec<WorldPos> = Vec::new();
    //for i in -20..=0 as i32 {
    //    for j in 1..=4 {
    //        let frame= ((i.abs() as u32) + j as u32) % 6;
    //        let world_pos = WorldPos {
    //            x: (i * 4) as f32,
    //            height: 0.0,
    //            depth: (j * 4) as f32,
    //        };
    //        grubling_positions.push(world_pos);
    //    }
    //}
    //let mut grublings = spawn_grublings(&grubling_sprite_clone, grubling_positions);

    let rabbit_spritesheet_image = 
    ggez::graphics::Image::from_path(&ctx, "/rabbit_idle.png")
        .expect("They bred like rabbits!");
    let rabbit_sprite_clone: Rc<graphics::Image> = Rc::new(rabbit_spritesheet_image);

    let mut rabbits: Vec<AnimatedRenderable> = Vec::new();
    for i in  1..=20 as i32 {
        for j in 1..=4 {
            let rabbit = AnimatedRenderable { 
                sprite: Spritesheet {
                    image: rabbit_sprite_clone.clone(),
                    frame: ((i.abs() as u32) + j as u32) % 21, // which frame you are on
                    sprite_width: 16, // width of a single frame
                    sprite_height: 16, // height of a single frame
                    hor_frames: 21, // how many frames horizontally
                    total_frames: 21,
                },
                anim_time: (((i.abs() as u32) + j as u32) % 21) as f32,
                anim_speed: 6.0, // how many frames a second to animate
                world_pos: WorldPos {
                    x: (i * 4) as f32,
                    height: 0.0,
                    depth: (j * 4) as f32,
                }
            };
            rabbits.push(rabbit);
        }
    }
    animated_renderables.append(&mut grublings);
    animated_renderables.append(&mut rabbits);


    let zindexed_renderables = BTreeMap::new();
        
    
    let mountain_background_sprite = 
        ggez::graphics::Image::from_path(&ctx, "/mountain.png")
        .expect("Over the Misty Mountains cold!");


    let state = State {
        is_batching : true,
        man_sprite_for_batch_test,
        grass_sprite,
        mountain_background_sprite,
        is_drawing_grubling: true,
        animated_renderables,
        dt: std::time::Duration::new(0, 0),
        renderables: men,
        playerpos: 0.0,
        playerspeed: 0.0,
        parallax_info: build_parallax_info(&ctx),
        zindexed_renderables,
    };
    event::run(ctx, event_loop, state);
}

fn spawn_man(sprite: &std::rc::Rc<graphics::Image>, world_pos: WorldPos) -> Renderable {
    Renderable {
        sprite: Rc::clone(sprite),
        world_pos,
    }
}

fn spawn_men(sprite: &std::rc::Rc<graphics::Image>, men_positions: Vec<WorldPos>) -> Vec<Renderable> {
    let mut men: Vec<Renderable> = Vec::new();
    for man_pos in men_positions.into_iter() {
        let man = spawn_man(sprite, man_pos);
        men.push(man);
    }
    men
}

fn spawn_grubling(sprite: &std::rc::Rc<graphics::Image>, world_pos: WorldPos, frame: u32) -> AnimatedRenderable {
    AnimatedRenderable { 
        sprite: Spritesheet {
            image: sprite.clone(),
            frame, // which frame you are on
            sprite_width: 32, // width of a single frame
            sprite_height: 32, // height of a single frame
            hor_frames: 2, // how many frames horizontally
            total_frames: 6,
        },
        world_pos,
        anim_time: frame as f32,
        anim_speed: 6.0, // how many frames a second to animate
    }
}

fn spawn_grublings(sprite: &std::rc::Rc<graphics::Image>, grubling_positions: Vec<WorldPos>) -> Vec<AnimatedRenderable> {
    let mut grublings: Vec<AnimatedRenderable> = Vec::new();
    for grubling_pos in grubling_positions.into_iter() {
        let frame = ((grubling_pos.x.abs() as u32) + grubling_pos.depth as u32) % 6;
        let grubling = spawn_grubling(sprite, grubling_pos, frame);
        grublings.push(grubling);
    }
    grublings
}

fn spawn_grid_of_grublings(sprite: &std::rc::Rc<graphics::Image>, x: i32, depth: i32, offset_x: i32) -> Vec<AnimatedRenderable> {
    let mut grubling_positions: Vec<WorldPos> = Vec::new();
    for x in 0 + offset_x ..x + offset_x {
        for depth in 1..depth {
            let world_pos = WorldPos {
                x: (x * 4) as f32,
                height: 0.0,
                depth: (depth * 4) as f32,
            };
            grubling_positions.push(world_pos);
        }
    }
    let grublings = spawn_grublings(&sprite, grubling_positions);
    grublings
}

fn spawn_unit(sprite: &std::rc::Rc<graphics::Image>, world_pos: WorldPos, frame: u32) -> AnimatedRenderable {
    AnimatedRenderable { 
        sprite: Spritesheet {
            image: sprite.clone(),
            frame, // which frame you are on
            sprite_width: 32, // width of a single frame
            sprite_height: 32, // height of a single frame
            hor_frames: 2, // how many frames horizontally
            total_frames: 6,
        },
        world_pos,
        anim_time: frame as f32,
        anim_speed: 6.0, // how many frames a second to animate
    }
}


fn spawn_units(sprite: &std::rc::Rc<graphics::Image>, unit_positions: Vec<WorldPos>) -> Vec<AnimatedRenderable> {
    let mut units: Vec<AnimatedRenderable> = Vec::new();
    for unit_pos in unit_positions.into_iter() {
        let frame = ((unit_pos.x.abs() as u32) + unit_pos.depth as u32) % 6;
        let unit = spawn_grubling(sprite, unit_pos, frame);
        units.push(unit);
    }
    units
}


fn spawn_grid_of_units(sprite: &std::rc::Rc<graphics::Image>, x: i32, depth: i32, offset_x: i32) -> Vec<AnimatedRenderable> {
    let mut unit_positions: Vec<WorldPos> = Vec::new();
    for x in 0 + offset_x ..x + offset_x {
        for depth in 1..depth {
            let world_pos = WorldPos {
                x: (x * 4) as f32,
                height: 0.0,
                depth: (depth * 4) as f32,
            };
            unit_positions.push(world_pos);
        }
    }
    let units = spawn_units(&sprite, unit_positions);
    units
}

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }
// impl Direction {
//     pub fn from_keycode(key: KeyCode) -> Option<Direction> {
//         match key {
//             KeyCode::Up => Some(Direction::Up),
//             KeyCode::Down => Some(Direction::Down),
//             KeyCode::Left => Some(Direction::Left),
//             KeyCode::Right => Some(Direction::Right),
//             _ => None,
//         }
//     }
// }
