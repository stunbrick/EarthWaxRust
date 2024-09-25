use ggez::*;
use std::rc::Rc;
pub fn main() {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        std::path::PathBuf::from("./assets")
    };
    let mut c = conf::Conf::new();
    c.window_mode.width = 1920.0;
    c.window_mode.height = 1080.0;
    let (ctx, event_loop) = ContextBuilder::new("earthwax", "broskisChimes")
        .default_conf(c)
        .add_resource_path(resource_dir)
        .build()
        .expect("Holy fuck I lost all context");
    let mut chickens = Vec::new();
    let chicken_sprite = ggez::graphics::Image::from_path(&ctx, "/chicken_idle.png").expect("Holy fuck no chicken_sprite!");
    let chicken_sprite_clone = Rc::new(chicken_sprite);
    for i in 0..100 {
        let chicken = Renderable {
            sprite: Rc::clone(&chicken_sprite_clone),
            world_pos: WorldPos {
                x: -20.0,
                height: 0.0,
                depth: i as f32,
            }
        };
        chickens.push(chicken);
    }
    let state = State {
        dt: std::time::Duration::new(0, 0),
        
        renderables: chickens
    };
    println!("Hello, world!");
    event::run(ctx, event_loop, state);
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


struct State {
    dt: std::time::Duration,
    renderables: Vec<Renderable>,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        let speed = 5.0;
        let delta_seconds = self.dt.as_secs_f32();
        for renderable in &mut self.renderables {
            renderable.world_pos.x += speed * delta_seconds;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::WHITE);
        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());

        for renderable in &self.renderables {
            canvas.draw(&*renderable.sprite, ggez::graphics::DrawParam::new().dest(render_pos(&renderable.world_pos)).scale([4.0, 4.0]));
        }
        // Draw code here...
        println!("Hello ggez! dt = {}ns", self.dt.as_nanos());
        canvas.finish(ctx)
    }
}

#[allow(non_snake_case)]
fn render_pos(world_pos: &WorldPos)->ggez::glam::Vec2 {
    let SCREEN_MAX_X = 1920.0;
    let SCREEN_MAX_Y = 1080.0;
    let HORIZON_ACTUAL = 420.0; // Where the sky meets land
    let HORIZON = HORIZON_ACTUAL - 50.0; // Where the infinity point is
    let SCREEN_MID_X = SCREEN_MAX_X / 2.0;
    let Z_ORIGIN_Y_OFFSET = SCREEN_MAX_Y - 458.0; // Where the first layer starts
    let LAND_PROJECTION_HEIGHT = Z_ORIGIN_Y_OFFSET - HORIZON;
    let X_UNIT =  32.0; // Width in  pixels at z0 to separate
    let Z_UNIT = 0.05; // Separation degree for z
    let Y_UNIT = 40.0;
    let y = HORIZON + (LAND_PROJECTION_HEIGHT + Y_UNIT * world_pos.height) / (world_pos.depth * Z_UNIT + 1.0);
    let x = (world_pos.x * X_UNIT) / (world_pos.depth * Z_UNIT + 1.0) + SCREEN_MID_X;
    ggez::glam::Vec2::new(x, y)
}

//func position_stuff_on_screen(delta): 
//	for parallax_obj in parallax_objects:
//		parallax_obj.visible = true
//		parallax_obj.position.x = z_and_x_to_x_converter(player_real_pos_x, 
//				parallax_obj.real_pos.z, parallax_obj.real_pos.x)
//		parallax_obj.position.y = y_and_z_to_y_converter(parallax_obj.real_pos.y, parallax_obj.real_pos.z)
//
//
//
//func y_and_z_to_y_converter(y_pos, z_pos):
//	z_pos = z_pos * Z_UNIT + 1
//	return (HORIZON + (HORIZON_HEIGHT + 40*y_pos) / z_pos)
//
//func z_and_x_to_x_converter(hero_x_pos, z_pos, x_pos):
//	x_pos = (x_pos + hero_x_pos) * X_UNIT * 1.0
//	z_pos = z_pos * Z_UNIT + 1
//	return SCREEN_MID_X + x_pos / z_pos
