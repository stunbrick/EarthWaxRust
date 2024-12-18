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
    pub world_pos: WorldPos,
} 

pub struct State {
    pub man_sprite_for_batch_test: ggez::graphics::Image,
    pub grass_sprite: ggez::graphics::Image,
    pub mountain_background_sprite: ggez::graphics::Image,
    pub is_drawing_grubling: bool, // Is drawing the grubling spritesheet for animation test
    pub animated_renderables: Vec<AnimatedRenderable>, // grubling spritesheets for animation test
    pub dt: std::time::Duration,
    pub playerpos: f32,
    pub playerspeed: f32,
    pub renderables: Vec<Renderable>,
    pub parallax_info: ParallaxInfo,
    pub is_batching: bool, // should batch sprites for faster display?
    pub zindexed_renderables: BTreeMap<i32, Vec<Renderable>>,
    pub sprite_master_clones: HashMap<SpriteUnit, Rc<graphics::Image>>,
}

#[derive(Hash, Eq, PartialEq)]
pub enum SpriteUnit {
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
    // Other man-specific animations
}
