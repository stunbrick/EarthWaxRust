use ggez::*;
use std::rc::Rc;
use ggez::{
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use ggez::{
    event,
    glam::*,
    graphics::{self, Color, PxScale, Text, TextAlign, TextFragment},
};

const SCREEN_MAX_X: f32 = 1920.0;
const SCREEN_MAX_Y: f32 = 1080.0;
const HORIZON_ACTUAL: f32 = 420.0; // Where the sky meets land
const HORIZON: f32 = HORIZON_ACTUAL - 50.0; // Where the infinity point is
const SCREEN_MID_X: f32 = SCREEN_MAX_X / 2.0;
const Z_ORIGIN_Y_OFFSET: f32 = SCREEN_MAX_Y - 200.0; // - 458.0; // Where the first layer starts
const LAND_PROJECTION_HEIGHT: f32 = Z_ORIGIN_Y_OFFSET - HORIZON;
const X_UNIT: f32 =  32.0; // Width in  pixels at z0 to separate
const Z_UNIT: f32 = 0.05; // Separation degree for z
const Y_UNIT: f32 = 40.0;

const SCREEN_QUART_X: f32 = SCREEN_MAX_X/4.0;

// in-game x-axis bounds within which to display
const CULL_WORLD_X_FULLSCREEN : f32 = 56.0;
const CULL_WORLD_X_HALFSCREEN : f32 = 32.0;

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
        parallax_info: ParallaxInfo {
            is_splitscreen: true,
            splitscreen_back_mesh: build_splitscreen_main_mesh(&ctx),
            splitscreen_parallax_mesh: build_splitscreen_parallax_mesh(&ctx),
            back_mesh: build_back_mesh(&ctx),
            parallax_mesh: build_parallax_mesh(&ctx),
        }
    };
    event::run(ctx, event_loop, state);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

struct WorldPos {
    x: f32,
    height: f32,
    depth: f32,
}

struct Renderable {
    sprite: Rc<ggez::graphics::Image>,
    world_pos: WorldPos,
}

struct ParallaxInfo { 
    is_splitscreen: bool,
    splitscreen_back_mesh: GameResult<graphics::Mesh>,
    splitscreen_parallax_mesh: GameResult<graphics::Mesh>,
    back_mesh: GameResult<graphics::Mesh>,
    parallax_mesh: GameResult<graphics::Mesh>,
}

struct State {
    dt: std::time::Duration,
    playerpos: f32,
    playerspeed: f32,
    renderables: Vec<Renderable>,
    parallax_info: ParallaxInfo,
}

impl State { 
    fn draw_parallax(&mut self, ctx: &mut Context) -> GameResult { 

        let parallax_info = &self.parallax_info;
        
        let mut background_canvas = 
            ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::GREEN);
        if let Ok(main_mesh) = &parallax_info.back_mesh {
            background_canvas.draw(main_mesh, graphics::DrawParam::new());
        }

        let mut canvas = 
            ggez::graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());

        if let Ok(mesh) = &parallax_info.parallax_mesh {
            background_canvas.draw(mesh, graphics::DrawParam::new());
        }

        for renderable in &self.renderables {
            if renderable.world_pos.x > self.playerpos-CULL_WORLD_X_FULLSCREEN && renderable.world_pos.x < self.playerpos+CULL_WORLD_X_FULLSCREEN { 
                canvas.draw(&*renderable.sprite, ggez::graphics::DrawParam::new()
                    .z((&renderable.world_pos.depth * -10.0) as i32)
                    .dest(render_pos(&renderable.world_pos, &self.playerpos, SCREEN_MID_X))
                    .offset([0.50, 0.91])
                    .scale([4.0, 4.0]));
            }
        }

        let fps = ctx.time.fps();
        let fps_display = Text::new(format!("FPS: {fps}"));
        canvas.draw(
            &fps_display,
            graphics::DrawParam::from([200.0, 0.0]).color(Color::BLACK),
        );
        background_canvas.finish(ctx)?;
        canvas.finish(ctx)
    }

    fn draw_splitscreen(&mut self, ctx: &mut Context) -> GameResult { 
        let parallax_info = &self.parallax_info;
        
        let mut background_canvas = 
            ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::GREEN);
        if let Ok(main_mesh) = &parallax_info.splitscreen_back_mesh {
            background_canvas.draw(main_mesh, graphics::DrawParam::new());
        }

        let mut canvas = 
            ggez::graphics::Canvas::from_frame(ctx, None);
        let rect = ggez::graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: SCREEN_MID_X,
            h: SCREEN_MAX_Y,
        };
        canvas.set_scissor_rect(rect)?;
        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());

        let mut canvas2 = 
            ggez::graphics::Canvas::from_frame(ctx, None);
        let rect2 = ggez::graphics::Rect {
            x: SCREEN_MID_X,
            y: 0.0,
            w: SCREEN_MAX_X,
            h: SCREEN_MAX_Y,
        };
        canvas2.set_scissor_rect(rect2)?;
        canvas2.set_sampler(ggez::graphics::Sampler::nearest_clamp());


        if let Ok(mesh) = &parallax_info.splitscreen_parallax_mesh {
            background_canvas.draw(mesh, graphics::DrawParam::new());
        }

        for renderable in &self.renderables {
            if renderable.world_pos.x > self.playerpos-CULL_WORLD_X_HALFSCREEN && renderable.world_pos.x < self.playerpos+CULL_WORLD_X_HALFSCREEN { 
                canvas.draw(&*renderable.sprite, ggez::graphics::DrawParam::new()
                    .z((&renderable.world_pos.depth * -10.0) as i32)
                    .dest(render_pos(&renderable.world_pos, &self.playerpos,  SCREEN_MID_X -SCREEN_QUART_X))
                    .offset([0.50, 0.91])
                    .scale([4.0, 4.0]));

                canvas2.draw(&*renderable.sprite, ggez::graphics::DrawParam::new()
                    .z((&renderable.world_pos.depth * -10.0) as i32)
                    .dest(render_pos2(&renderable.world_pos, &self.playerpos, SCREEN_MID_X + SCREEN_QUART_X))
                    .offset([0.50, 0.91])
                    .scale([4.0, 4.0]));
            }
        }

        let fps = ctx.time.fps();
        let fps_display = Text::new(format!("FPS: {fps}"));
        canvas.draw(
            &fps_display,
            graphics::DrawParam::from([200.0, 0.0]).color(Color::BLACK),
        );
        background_canvas.finish(ctx)?;
        canvas.finish(ctx)?;
        canvas2.finish(ctx)
    }
} 

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





#[allow(non_snake_case)]
fn render_pos(world_pos: &WorldPos, playerx: &f32, midpoint: f32)->ggez::glam::Vec2 {
    let y = HORIZON + (LAND_PROJECTION_HEIGHT + Y_UNIT * world_pos.height) / (world_pos.depth * Z_UNIT + 1.0);
    let x = ((world_pos.x - playerx) * X_UNIT) / (world_pos.depth * Z_UNIT + 1.0) + midpoint;
    ggez::glam::Vec2::new(x, y)
}

const Z_UNIT_TOP: f32 = 32.0; // Separation degree for z in top-down view

#[allow(non_snake_case)]
fn render_pos2(world_pos: &WorldPos, playerx: &f32, midpoint: f32)->ggez::glam::Vec2 {
    let y = Z_ORIGIN_Y_OFFSET - (world_pos.depth * Z_UNIT_TOP);
    let x = (world_pos.x - playerx) * X_UNIT + midpoint;
    ggez::glam::Vec2::new(x, y)
}

fn ver_line_full(mb: &mut graphics::MeshBuilder, x: f32, c: Color) -> Result<&mut graphics::MeshBuilder, GameError> { 
    mb.line(
        &[
            Vec2::new(x, 0.0),
            Vec2::new(x, SCREEN_MAX_Y),
        ],
        4.0,
        c
    )
}

fn hor_line_half(mb: &mut graphics::MeshBuilder, y: f32, c: Color) -> Result<&mut graphics::MeshBuilder, GameError> { 
    mb.line(
        &[
            Vec2::new(0.0, y),
            Vec2::new(SCREEN_MID_X, y),
        ],
        4.0,
        c
    )
}

fn hor_line_full(mb: &mut graphics::MeshBuilder, y: f32, c: Color) -> Result<&mut graphics::MeshBuilder, GameError> { 
    mb.line(
        &[
            Vec2::new(0.0, y),
            Vec2::new(SCREEN_MAX_X, y),
        ],
        4.0,
        c
    )
}
const WHITE : Color = Color::new(1.0, 1.0, 1.0, 1.0);
const BLACK : Color = Color::new(0.0, 0.0, 0.0, 1.0);
const RED   : Color = Color::new(1.0, 0.0, 0.0, 1.0);

fn build_splitscreen_main_mesh(ctx: &Context) -> GameResult<graphics::Mesh> {
    let mb: &mut graphics::MeshBuilder = &mut graphics::MeshBuilder::new();

    ver_line_full(mb, SCREEN_QUART_X, WHITE)?;
    ver_line_full(mb, SCREEN_MID_X, BLACK)?;
    ver_line_full(mb, SCREEN_MID_X + SCREEN_QUART_X, WHITE)?;

    Ok(graphics::Mesh::from_data(ctx, mb.build()))
}


fn build_back_mesh(ctx: &Context) -> GameResult<graphics::Mesh> {
    let mb: &mut graphics::MeshBuilder = &mut graphics::MeshBuilder::new();
    ver_line_full(mb, SCREEN_MID_X, WHITE)?;
    Ok(graphics::Mesh::from_data(ctx, mb.build()))
}

fn build_parallax_mesh(ctx: &Context) -> GameResult<graphics::Mesh> {
    let mb: &mut graphics::MeshBuilder = &mut graphics::MeshBuilder::new();
    hor_line_full(mb, HORIZON_ACTUAL, RED)?;
    hor_line_full(mb, Z_ORIGIN_Y_OFFSET, RED)?;
    Ok(graphics::Mesh::from_data(ctx, mb.build()))
}

fn build_splitscreen_parallax_mesh(ctx: &Context) -> GameResult<graphics::Mesh> {
    let mb: &mut graphics::MeshBuilder = &mut graphics::MeshBuilder::new();
    hor_line_half(mb, HORIZON_ACTUAL, RED)?;
    hor_line_half(mb, Z_ORIGIN_Y_OFFSET, RED)?;
    Ok(graphics::Mesh::from_data(ctx, mb.build()))
}