use ggez::*;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::HashMap;
use ggez::graphics;

#[derive(Copy, Clone)]
pub struct WorldPos {
    pub x: f32,
    pub height: f32,
    pub depth: f32,
}

pub struct Renderable {
    pub sprite: Rc<ggez::graphics::Image>,
    pub world_pos: WorldPos,
}


pub struct BatchedRenderable {
    pub sprite: Rc<ggez::graphics::Image>,
    pub world_pos: WorldPos,
}


pub struct ParallaxInfo { 
    pub parallax_top_y: f32, // horizon, basically, on-screen,
    pub parallax_thickness_y: f32, // how far the parallax takes up on-screen
    pub z_sep_top: f32, // how much units are separated vertically in top-down view
    pub is_splitscreen: bool,
    // TODO These should be direct meshes and not game results
    pub splitscreen_back_mesh: GameResult<graphics::Mesh>,
    pub splitscreen_parallax_mesh: GameResult<graphics::Mesh>,
    pub back_mesh: GameResult<graphics::Mesh>,
    pub parallax_mesh: GameResult<graphics::Mesh>,
    pub background_color_index : u8,
}

#[derive(Clone, Copy)]
pub struct AnimatedSpriteInfo {
    pub frame: u32, // which frame you are on
    pub sprite_width: u32, // width of a single frame in pixels
    pub sprite_height: u32, // height of a single frame in pixels
    pub hor_frames: u32, // how many frames horizontally
    pub total_frames: u32, // how many frames total
}

pub enum AnimatedSprites {
    Grubling,
    Rabbit,
    RabbitRun,
}

impl AnimatedSprites {
    pub fn get_info (&self) -> AnimatedSpriteInfo {
        match self {
            AnimatedSprites::Grubling => AnimatedSpriteInfo {
                frame: 0,
                sprite_width: 32,
                sprite_height: 32,
                hor_frames: 2,
                total_frames: 6,
            },
            AnimatedSprites::Rabbit => AnimatedSpriteInfo {
                frame: 0,
                sprite_width: 16,
                sprite_height: 16,
                hor_frames: 21,
                total_frames: 21,
            },
            AnimatedSprites::RabbitRun => AnimatedSpriteInfo {
                frame: 0,
                sprite_width: 16,
                sprite_height: 16,
                hor_frames: 15,
                total_frames: 15,
            }
        }
    }
}



pub struct Spritesheet {
    pub image: Rc<ggez::graphics::Image>,
    pub frame: u32, // which frame you are on
    pub sprite_width: u32, // width of a single frame in pixels
    pub sprite_height: u32, // height of a single frame in pixels
    pub hor_frames: u32, // how many frames horizontally
    pub total_frames: u32, // how many frames total
}


pub struct AnimatedRenderable { 
    pub sprite: Spritesheet,
    pub anim_time: f32, // where in the animation you are
    pub anim_speed: f32,
} 

pub struct State {
    pub man_sprite_for_batch_test: ggez::graphics::Image,
    pub grass_sprite: ggez::graphics::Image,
    pub mountain_background_sprite: ggez::graphics::Image,
    pub is_drawing_grubling: bool, // Is drawing the grubling spritesheet for animation test
    pub units: Vec<Unit>,
    pub dt: std::time::Duration,
    pub playerpos: f32,
    pub playerspeed: f32,
    //pub renderables: Vec<Renderable>,
    pub parallax_info: ParallaxInfo,
    pub is_batching: bool, // should batch sprites for faster display?
    pub zindexed_renderables: BTreeMap<i32, Vec<Renderable>>,
    pub animation_system : AnimationSystem,
    pub done_once: bool,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Anim {
    Rabbit(RabbitAnim),
    Grubling(GrublingAnim),
    // Other units can be added here
}

#[derive(Hash, Eq, PartialEq)]
pub enum RabbitAnim {
    Idle,
    Run,
    // Other rabbit-specific animations
}

#[derive(Hash, Eq, PartialEq)]
pub enum GrublingAnim {
    Attack,
    Idle,
    // Other man-specific animations
}

pub enum UnitState {
    Idle,
    Move,
}

pub enum UnitType {
    Rabbit,
    Grubling,
}

pub struct Unit {
    pub animated_renderable: AnimatedRenderable,
    pub state: UnitState,
    pub world_pos: WorldPos,
}

pub struct AnimationSystem {
    pub sprite_master_clones: HashMap<Anim, Rc<graphics::Image>>,
    pub sprite_master_info: HashMap<Anim, AnimatedSpriteInfo>
}

impl AnimationSystem {
    pub fn animate_units(&mut self, units: &mut Vec<Unit>, delta_seconds: f32) {
        for unit in units {
            let y: &mut f32 = &mut unit.animated_renderable.anim_time;
            *y = *y + unit.animated_renderable.anim_speed * delta_seconds;
            while *y > unit.animated_renderable.sprite.total_frames as f32 {
                *y = *y-unit.animated_renderable.sprite.total_frames as f32; 
            }

            let x: &mut u32 = &mut unit.animated_renderable.sprite.frame;
            *x = *y as u32;
        }
    }
    pub fn get_sprite_and_info_for_unit(&self, unit_type: UnitType) -> (Rc<ggez::graphics::Image>, AnimatedSpriteInfo) {
        let anim = match unit_type {
            UnitType::Grubling => Anim::Grubling(GrublingAnim::Idle),
            UnitType::Rabbit => Anim::Rabbit(RabbitAnim::Idle),
        };
        let sprite = self.sprite_master_clones
            .get(&anim)
            .expect("oops no sprite for anim get");
        let sprite_info = self.sprite_master_info
            .get(&anim)
            .expect("oops no sprite info for anim info get");
        (sprite.clone(), sprite_info.clone())
    }
    pub fn new(ctx: &ggez::Context) -> AnimationSystem {
        let (sprite_master_clones, sprite_master_info) = Self::load_sprite_master_clones_and_info(ctx);
        AnimationSystem { sprite_master_clones, sprite_master_info }
    }
    fn load_sprite_master_clones_and_info(ctx: &ggez::Context) -> (HashMap<Anim, Rc<graphics::Image>>, HashMap<Anim,AnimatedSpriteInfo>) {
        let mut sprite_resources = HashMap::new();
        let mut sprite_info = HashMap::new();

        //Rabbit
        let rabbit_idle =
            graphics::Image::from_path(ctx, "/rabbit_idle.png")
            .expect("They bred like rabbits!");
        sprite_resources.insert(
            Anim::Rabbit(RabbitAnim::Idle),
            Rc::new(rabbit_idle)
        );
        sprite_info.insert(
            Anim::Rabbit(RabbitAnim::Idle) 
            AnimatedSpriteInfo {
                frame: 0,
                sprite_width: 16,
                sprite_height: 16,
                hor_frames: 21,
                total_frames: 21,
            }
        );

        let rabbit_run =
            graphics::Image::from_path(ctx, "/rabbit_sprint.png")
            .expect("Run like em too!");
        sprite_resources.insert(
            Anim::Rabbit(RabbitAnim::Run),
            Rc::new(rabbit_run)
        );
        sprite_info.insert(
            Anim::Rabbit(RabbitAnim::Run) 
            AnimatedSpriteInfo {
                frame: 0,
                sprite_width: 16,
                sprite_height: 16,
                hor_frames: 15,
                total_frames: 15,
            },
        );

        //Grubling
        let grubling_idle =
            graphics::Image::from_path(ctx, "/grub_small_attack.png")
            .expect("Don't feed the grublings after midnight!");
        sprite_resources.insert(
            Anim::Grubling(GrublingAnim::Idle),
            Rc::new(grubling_idle),
        );
        sprite_info.insert(
            Anim::Grubling(GrublingAnim::Idle) 
            AnimatedSpriteInfo {
                frame: 0,
                sprite_width: 32,
                sprite_height: 32,
                hor_frames: 2,
                total_frames: 6,
            },
        );

        let grubling_attack =
            graphics::Image::from_path(ctx, "/grub_small_attack.png")
            .expect("Don't feed the grublings after midnight!");
        sprite_resources.insert(
            Anim::Grubling(GrublingAnim::Attack),
            Rc::new(grubling_attack),
        );
        sprite_info.insert(
            Anim::Grubling(GrublingAnim::Attack) 
            AnimatedSpriteInfo {
                frame: 0,
                sprite_width: 32,
                sprite_height: 32,
                hor_frames: 2,
                total_frames: 6,
            },
        );

        (sprite_resources, sprite_info)
    }
    pub fn change_unit_anim(&self, unit: &mut Unit, animation: Anim) {
        let sprite = self.sprite_master_clones
            .get(&animation)
            .expect("oops no sprite for anim change");
        let sprite_info = self.sprite_master_info
            .get(&animation)
            .expect("oops no sprite info for anim change");
        *unit = Unit {
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
            },
            world_pos: unit.world_pos,
            state: UnitState::Move,
        }
    }
}
