use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use ggez::graphics;

use crate::{
    build_parallax_info, AnimatedRenderable, AnimatedSpriteInfo, AnimatedSprites, GrublingAnim,
    RabbitAnim, Renderable, SpriteUnit, Spritesheet, State, WorldPos,
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
    let men = spawn_men(&man_sprite_clone, men_positions);

    let man_sprite_for_batch_test =
        ggez::graphics::Image::from_path(ctx, "/farmer_idle.png").expect("Holy Fuck, Batchman!");
    // let sprite_batch =  ggez::graphics::InstanceArray::new_ordered(&ctx, man_sprite_for_batch);

    let grass_sprite = ggez::graphics::Image::from_path(ctx, "/grass_small.png")
        .expect("Who smoked all the grass?!");

    let sprite_master_clones = load_sprite_master_clones(&ctx);

    let grubling_sprite_sheet_image =
        ggez::graphics::Image::from_path(ctx, "/grub_small_attack.png")
            .expect("Don't feed the grublings after midnight!");
    let grubling_sprite_clone: Rc<graphics::Image> = Rc::new(grubling_sprite_sheet_image);

    let mut animated_renderables: Vec<AnimatedRenderable> = Vec::new();

    let mut grublings = spawn_grid_of_units(
        sprite_master_clones
            .get(&SpriteUnit::Grubling(GrublingAnim::Attack))
            .expect("oops no grubby sprite"),
        AnimatedSprites::Grubling.get_info(),
        20,
        4,
        -20,
    );

    let rabbit_spritesheet_image = ggez::graphics::Image::from_path(ctx, "/rabbit_idle.png")
        .expect("They bred like rabbits!");
    let rabbit_sprite_clone: Rc<graphics::Image> = Rc::new(rabbit_spritesheet_image);
    let mut rabbits = spawn_grid_of_units(
        &rabbit_sprite_clone,
        AnimatedSprites::Rabbit.get_info(),
        20,
        4,
        0,
    );

    let rabbit_run_spritesheet_image = ggez::graphics::Image::from_path(ctx, "/rabbit_sprint.png")
        .expect("They bred like rabbits!");
    let rabbit_run_sprite_clone: Rc<graphics::Image> = Rc::new(rabbit_run_spritesheet_image);
    for rabbit in &mut rabbits {
        change_animation(
            rabbit,
            &rabbit_run_sprite_clone.clone(),
            AnimatedSprites::RabbitRun.get_info(),
        );
    }

    animated_renderables.append(&mut grublings);
    animated_renderables.append(&mut rabbits);

    let zindexed_renderables = BTreeMap::new();

    let mountain_background_sprite = ggez::graphics::Image::from_path(ctx, "/mountain.png")
        .expect("Over the Misty Mountains cold!");

    State {
        is_batching: true,
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
        sprite_master_clones,
    }
}

fn spawn_man(sprite: &std::rc::Rc<graphics::Image>, world_pos: WorldPos) -> Renderable {
    Renderable {
        sprite: Rc::clone(sprite),
        world_pos,
    }
}

fn spawn_men(
    sprite: &std::rc::Rc<graphics::Image>,
    men_positions: Vec<WorldPos>,
) -> Vec<Renderable> {
    let mut men: Vec<Renderable> = Vec::new();
    for man_pos in men_positions.into_iter() {
        let man = spawn_man(sprite, man_pos);
        men.push(man);
    }
    men
}

fn spawn_grubling(
    sprite: &std::rc::Rc<graphics::Image>,
    world_pos: WorldPos,
    frame: u32,
) -> AnimatedRenderable {
    AnimatedRenderable {
        sprite: Spritesheet {
            image: sprite.clone(),
            frame,             // which frame you are on
            sprite_width: 32,  // width of a single frame
            sprite_height: 32, // height of a single frame
            hor_frames: 2,     // how many frames horizontally
            total_frames: 6,
        },
        world_pos,
        anim_time: frame as f32,
        anim_speed: 6.0, // how many frames a second to animate
    }
}

fn spawn_grublings(
    sprite: &std::rc::Rc<graphics::Image>,
    grubling_positions: Vec<WorldPos>,
) -> Vec<AnimatedRenderable> {
    let mut grublings: Vec<AnimatedRenderable> = Vec::new();
    for grubling_pos in grubling_positions.into_iter() {
        let frame = ((grubling_pos.x.abs() as u32) + grubling_pos.depth as u32) % 6;
        let grubling = spawn_grubling(sprite, grubling_pos, frame);
        grublings.push(grubling);
    }
    grublings
}

fn spawn_grid_of_grublings(
    sprite: &std::rc::Rc<graphics::Image>,
    x: i32,
    depth: i32,
    offset_x: i32,
) -> Vec<AnimatedRenderable> {
    let mut grubling_positions: Vec<WorldPos> = Vec::new();
    for x in 0 + offset_x..x + offset_x {
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

fn spawn_unit(
    sprite: &std::rc::Rc<graphics::Image>,
    sprite_info: AnimatedSpriteInfo,
    world_pos: WorldPos,
) -> AnimatedRenderable {
    AnimatedRenderable {
        sprite: Spritesheet {
            image: sprite.clone(),
            frame: sprite_info.frame,               // which frame you are on
            sprite_width: sprite_info.sprite_width, // width of a single frame
            sprite_height: sprite_info.sprite_height, // height of a single frame
            hor_frames: sprite_info.hor_frames,     // how many frames horizontally
            total_frames: sprite_info.total_frames,
        },
        world_pos,
        anim_time: sprite_info.frame as f32,
        anim_speed: 6.0, // how many frames a second to animate
    }
}

fn spawn_units(
    sprite: &std::rc::Rc<graphics::Image>,
    sprite_info: AnimatedSpriteInfo,
    unit_positions: Vec<WorldPos>,
) -> Vec<AnimatedRenderable> {
    let mut units: Vec<AnimatedRenderable> = Vec::new();
    for unit_pos in unit_positions.into_iter() {
        let new_frame = ((unit_pos.x.abs() as u32) + unit_pos.depth as u32) % 6;
        let new_sprite_info = AnimatedSpriteInfo {
            frame: new_frame,
            sprite_width: sprite_info.sprite_width,
            sprite_height: sprite_info.sprite_height,
            hor_frames: sprite_info.hor_frames,
            total_frames: sprite_info.total_frames,
        };
        let unit = spawn_unit(sprite, new_sprite_info, unit_pos);
        units.push(unit);
    }
    units
}

fn spawn_grid_of_units(
    sprite: &std::rc::Rc<graphics::Image>,
    mut sprite_info: AnimatedSpriteInfo,
    x: i32,
    depth: i32,
    offset_x: i32,
) -> Vec<AnimatedRenderable> {
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
    let units = spawn_units(&sprite, sprite_info, unit_positions);
    units
}

fn change_animation(
    unit: &mut AnimatedRenderable,
    sprite: &std::rc::Rc<graphics::Image>,
    mut sprite_info: AnimatedSpriteInfo,
) {
    *unit = AnimatedRenderable {
        sprite: Spritesheet {
            image: sprite.clone(),
            frame: sprite_info.frame,               // which frame you are on
            sprite_width: sprite_info.sprite_width, // width of a single frame
            sprite_height: sprite_info.sprite_height, // height of a single frame
            hor_frames: sprite_info.hor_frames,     // how many frames horizontally
            total_frames: sprite_info.total_frames,
        },
        world_pos: unit.world_pos,
        anim_time: sprite_info.frame as f32,
        anim_speed: 6.0, // how many frames a second to animate
    }
}

fn load_sprite_master_clones(ctx: &ggez::Context) -> HashMap<SpriteUnit, Rc<graphics::Image>> {
    let mut sprite_resources = HashMap::new();
    //Rabbit
    let rabbit_idle =
        graphics::Image::from_path(ctx, "/rabbit_idle.png").expect("They bred like rabbits!");
    sprite_resources.insert(SpriteUnit::Rabbit(RabbitAnim::Idle), Rc::new(rabbit_idle));

    let rabbit_run =
        graphics::Image::from_path(ctx, "/rabbit_sprint.png").expect("Run like em too!");
    sprite_resources.insert(SpriteUnit::Rabbit(RabbitAnim::Run), Rc::new(rabbit_run));

    //Grubling
    let grubling_attack = graphics::Image::from_path(ctx, "/grub_small_attack.png")
        .expect("Don't feed the grublings after midnight!");
    sprite_resources.insert(
        SpriteUnit::Grubling(GrublingAnim::Attack),
        Rc::new(grubling_attack),
    );

    sprite_resources
}
