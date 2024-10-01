use ggez::*;
use std::rc::Rc;

pub struct WorldPos {
    pub x: f32,
    pub height: f32,
    pub depth: f32,
}

pub struct Renderable {
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
}

pub struct State {
    pub dt: std::time::Duration,
    pub playerpos: f32,
    pub playerspeed: f32,
    pub renderables: Vec<Renderable>,
    pub parallax_info: ParallaxInfo,
}




