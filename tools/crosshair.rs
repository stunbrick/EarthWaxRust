#![allow(unused)]
use ggez::*;
use ggez::{
    input::keyboard::{KeyCode, KeyInput, KeyMods},
    Context, GameResult,
    event,
    glam::*,
    graphics::{self, Color, PxScale, Text, TextAlign, TextFragment},
};
use clipboard_rs::Clipboard;

const SCREEN_MAX_X: f32 = 800.0;
const SCREEN_MAX_Y: f32 = 600.0;
const SCREEN_BOTTOM_QUARTER: f32 = 3.0 * (SCREEN_MAX_Y / 4.0);
const SCREEN_VERTICAL_MIDDLE: f32 = SCREEN_MAX_X / 2.0;


struct State {
    dt: std::time::Duration,
    sprite: ggez::graphics::Image,
    mesh: GameResult<graphics::Mesh>,
    vertical_offset: f32,
    horizontal_offset: f32,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        let delta_seconds = self.dt.as_secs_f32();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::WHITE);
        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());
        if let Ok(mesh) = &self.mesh {
            canvas.draw(mesh, graphics::DrawParam::new());
        }

        canvas.draw(&self.sprite, ggez::graphics::DrawParam::new()
            .dest(ggez::glam::Vec2::new(SCREEN_VERTICAL_MIDDLE, SCREEN_BOTTOM_QUARTER))
            .offset([self.horizontal_offset, self.vertical_offset])
            .scale([4.0, 4.0]));
        let fps = ctx.time.fps();
        let fps_display = Text::new(format!("FPS: {fps}"));
        canvas.draw(
            &fps_display,
            graphics::DrawParam::from([200.0, 0.0]).color(Color::BLACK),
        );
        let horizontal_offset_display = Text::new(format!("Horizontal: {0}", self.horizontal_offset));
        canvas.draw(
            &horizontal_offset_display,
            graphics::DrawParam::from([600.0, 100.0]).color(Color::BLACK),
        );
        let vertical_offset_display = Text::new(format!("Vertical: {0}", self.vertical_offset));
        canvas.draw(
            &vertical_offset_display,
            graphics::DrawParam::from([600.0, 200.0]).color(Color::BLACK),
        );
        canvas.finish(ctx)
    }
    fn key_down_event(&mut self, ctx: &mut Context, input: ggez::input::keyboard::KeyInput, _repeat: bool) -> GameResult {
        let mut arrow_speed = 0.2;
        let mut wasd_speed = 1.0;
        if (ctx.keyboard.is_mod_active(KeyMods::SHIFT)) {
            arrow_speed *= 5.0;
            wasd_speed *= 5.0;
        }
        let arrow_diff = ctx.time.delta().as_secs_f32() * arrow_speed;
        let wasd_diff = ctx.time.delta().as_secs_f32() * wasd_speed;
        if let Some(key) = input.keycode {
            match key {
                KeyCode::Escape | KeyCode::Q => ctx.request_quit(),
                KeyCode::Left => self.horizontal_offset += arrow_diff,
                KeyCode::Right => self.horizontal_offset -= arrow_diff,
                KeyCode::Up => self.vertical_offset += arrow_diff,
                KeyCode::Down => self.vertical_offset -= arrow_diff,
                KeyCode::W => self.vertical_offset += wasd_diff,
                KeyCode::A => self.horizontal_offset += wasd_diff,
                KeyCode::S => self.vertical_offset -= wasd_diff,
                KeyCode::D => self.horizontal_offset -= wasd_diff,
                KeyCode::Return => {
                    let clip = clipboard_rs::ClipboardContext::new().expect("NO CLIPBOARD?!");
                    let out_str = format!("{}, {}", self.horizontal_offset, self.vertical_offset);
                    clip.set_text(out_str);

                },
                _ => (),
            }
        }
        Ok(())
    }
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        std::path::PathBuf::from("./assets")
    };
    let mut c = conf::Conf::new();
    c.window_mode.width = 800.0;
    c.window_mode.height = 600.0;
    let (ctx, event_loop) = ContextBuilder::new("crosshair", "broskisChimes")
        .default_conf(c)
        .add_resource_path(resource_dir)
        .build()
        .expect("Holy fuck I lost all context");
    let sprite = ggez::graphics::Image::from_path(&ctx, "/farmer_idle.png").expect("Holy fuck no sprite!");
    let state = State {
        dt: std::time::Duration::new(0, 0),
        sprite,
        mesh: build_mesh(&ctx),
        vertical_offset: 0.0,
        horizontal_offset: 0.0,
    };
    event::run(ctx, event_loop, state);
}

fn build_mesh(ctx: &Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();
    mb.line(
        &[
            Vec2::new(0.0, SCREEN_BOTTOM_QUARTER),
            Vec2::new(SCREEN_MAX_X, SCREEN_BOTTOM_QUARTER),
        ],
        4.0,
        Color::new(1.0, 0.0, 0.0, 1.0),
    )?;
    mb.line(
        &[
            Vec2::new(SCREEN_VERTICAL_MIDDLE, 0.0),
            Vec2::new(SCREEN_VERTICAL_MIDDLE, SCREEN_MAX_Y),
        ],
        4.0,
        Color::new(1.0, 0.0, 0.0, 1.0),
    )?;
    Ok(graphics::Mesh::from_data(ctx, mb.build()))
}
