use std::{
    collections::{BTreeMap},
    rc::Rc,
};

use ggez::graphics;

use crate::{
    build_parallax_info, AnimatedRenderable, AnimatedSpriteInfo, Renderable, Spritesheet, State, WorldPos, Unit, UnitState,
    AnimationSystem, UnitType,
};

pub fn setup_grids(ctx: & ggez::context::Context) -> State {
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

    let man_sprite = ggez::graphics::Image::from_path(ctx, "/farmer_idle.png")
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

    let man_sprite_for_batch_test =
        ggez::graphics::Image::from_path(ctx, "/farmer_idle.png").expect("Holy Fuck, Batchman!");
    // let sprite_batch =  ggez::graphics::InstanceArray::new_ordered(&ctx, man_sprite_for_batch);

    let grass_sprite = ggez::graphics::Image::from_path(ctx, "/grass_small.png")
        .expect("Who smoked all the grass?!");

    let animation_system = AnimationSystem::new(&ctx);

//    let grubling_sprite_sheet_image =
//        ggez::graphics::Image::from_path(ctx, "/grub_small_attack.png")
//            .expect("Don't feed the grublings after midnight!");
//    let grubling_sprite_clone: Rc<graphics::Image> = Rc::new(grubling_sprite_sheet_image);
//
//    let mut units: Vec<Unit> = Vec::new();
//
//    let mut grublings = spawn_grid_of_units(
//        animation_system.sprite_master_clones
//            .get(&Anim::Grubling(GrublingAnim::Attack))
//            .expect("oops no grubby sprite"),
//        AnimatedSprites::Grubling.get_info(),
//        20,
//        4,
//        -20,
//    );
//
//    let rabbit_spritesheet_image = ggez::graphics::Image::from_path(ctx, "/rabbit_idle.png")
//        .expect("They bred like rabbits!");
//    let rabbit_sprite_clone: Rc<graphics::Image> = Rc::new(rabbit_spritesheet_image);
//    let mut rabbits = spawn_grid_of_units(
//        animation_system.sprite_master_clones
//            .get(&Anim::Rabbit(RabbitAnim::Idle))
//            .expect("oops no rabbit sprite"),
//        AnimatedSprites::Rabbit.get_info(),
//        20,
//        4,
//        0,
//    );
//
//    let rabbit_run_spritesheet_image = ggez::graphics::Image::from_path(ctx, "/rabbit_sprint.png")
//        .expect("They bred like rabbits!");
//    let rabbit_run_sprite_clone: Rc<graphics::Image> = Rc::new(rabbit_run_spritesheet_image);
//    let mut i = 0;
//    for rabbit in &mut rabbits {
//        i += 1;
//        if i%2 == 0 {
//
//            change_animation(
//                rabbit,
//                &rabbit_run_sprite_clone.clone(),
//                AnimatedSprites::RabbitMove.get_info(),
//                );
//        }
//    }
//
//    units.append(&mut grublings);
//    units.append(&mut rabbits);

    let zindexed_renderables = BTreeMap::new();

    let mountain_background_sprite = ggez::graphics::Image::from_path(ctx, "/mountain.png")
        .expect("Over the Misty Mountains cold!");

    State {
        is_batching: true,
        man_sprite_for_batch_test,
        grass_sprite,
        mountain_background_sprite,
        is_drawing_grubling: true,
        units: Vec::<Unit>::new(),
        dt: std::time::Duration::new(0, 0),
        //renderables: men,
        playerpos: 0.0,
        playerspeed: 0.0,
        parallax_info: build_parallax_info(&ctx),
        zindexed_renderables,
        animation_system,
        done_once: false,
    }
}
//fn spawn_unit(
//    unit_type: UnitType,
//    sprite: &std::rc::Rc<graphics::Image>,
//    sprite_info: AnimatedSpriteInfo,
//    world_pos: WorldPos,
//) -> Unit {
//    Unit {
//        unit_type,
//        animated_renderable: AnimatedRenderable {
//            sprite: Spritesheet {
//                image: sprite.clone(),
//                frame: sprite_info.frame,               // which frame you are on
//                sprite_width: sprite_info.sprite_width, // width of a single frame
//                sprite_height: sprite_info.sprite_height, // height of a single frame
//                hor_frames: sprite_info.hor_frames,     // how many frames horizontally
//                total_frames: sprite_info.total_frames,
//            },
//            anim_time: sprite_info.frame as f32,
//            anim_speed: 6.0, // how many frames a second to animate
//            flip_x: false,
//        },
//        world_pos,
//        destination: world_pos,
//        state: UnitState::Idle
//    }
//}

//fn spawn_units(
//    unit_type: UnitType,
//    sprite: &std::rc::Rc<graphics::Image>,
//    sprite_info: AnimatedSpriteInfo,
//    unit_positions: Vec<WorldPos>,
//) -> Vec<Unit> {
//    let mut units: Vec<Unit> = Vec::new();
//    for unit_pos in unit_positions.into_iter() {
//        let new_frame = ((unit_pos.x.abs() as u32) + unit_pos.depth as u32) % 6;
//        let new_sprite_info = AnimatedSpriteInfo {
//            frame: new_frame,
//            sprite_width: sprite_info.sprite_width,
//            sprite_height: sprite_info.sprite_height,
//            hor_frames: sprite_info.hor_frames,
//            total_frames: sprite_info.total_frames,
//        };
//        let unit = spawn_unit(unit_type, sprite, new_sprite_info, unit_pos);
//        units.push(unit);
//    }
//    units
//}
//
//fn spawn_grid_of_units(
//    unit_type: UnitType,
//    sprite: &std::rc::Rc<graphics::Image>,
//    mut sprite_info: AnimatedSpriteInfo,
//    x: i32,
//    depth: i32,
//    offset_x: i32,
//) -> Vec<Unit> {
//    let mut unit_positions: Vec<WorldPos> = Vec::new();
//    for x in 0 + offset_x..x + offset_x {
//        for depth in 1..depth {
//            let world_pos = WorldPos {
//                x: (x * 4) as f32,
//                height: 0.0,
//                depth: (depth * 4) as f32,
//            };
//            unit_positions.push(world_pos);
//        }
//    }
//    let units = spawn_units(unit_type, &sprite, sprite_info, unit_positions);
//    units
//}
//
//fn change_animation(
//    unit: &mut Unit,
//    sprite: &std::rc::Rc<graphics::Image>,
//    mut sprite_info: AnimatedSpriteInfo,
//) {
//    *unit = Unit {
//        unit_type: unit.unit_type,
//        animated_renderable: AnimatedRenderable {
//            sprite: Spritesheet {
//                image: sprite.clone(),
//                frame: sprite_info.frame,               // which frame you are on
//                sprite_width: sprite_info.sprite_width, // width of a single frame
//                sprite_height: sprite_info.sprite_height, // height of a single frame
//                hor_frames: sprite_info.hor_frames,     // how many frames horizontally
//                total_frames: sprite_info.total_frames,
//            },
//            anim_time: sprite_info.frame as f32,
//            anim_speed: 6.0, // how many frames a second to animate
//            flip_x: false,
//        },
//        world_pos: unit.world_pos,
//        destination: unit.destination,
//        state: UnitState::Move,
//    }
//}

