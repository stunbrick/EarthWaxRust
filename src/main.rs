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
    let mut men = Vec::new();
    let man_sprite = 
        ggez::graphics::Image::from_path(&ctx, "/farmer_idle.png")
        .expect("Holy fuck no man_sprite!");

    let man_sprite_clone = Rc::new(man_sprite);
    for i in -5..=5 {
        for j in 0..4 {
            let man = Renderable {
                sprite: Rc::clone(&man_sprite_clone),
                world_pos: WorldPos {
                    x: (i * 4) as f32,
                    height: 0.0,
                    depth: (j * 4) as f32,
                }
            };
            men.push(man);
        }
    }
    let state = State {
        dt: std::time::Duration::new(0, 0),
        renderables: men,
        playerpos: 0.0,
        playerspeed: 0.0,
        parallax_info: build_parallax_info(&ctx),
    };
    event::run(ctx, event_loop, state);
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
