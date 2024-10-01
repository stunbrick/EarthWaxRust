use ggez::*;

use crate::structs::{ParallaxInfo, State, WorldPos};
use crate::constants::*;

use ggez::{
    Context, GameResult,
};
use ggez::{
    glam::*,
    graphics::{self, Color, Text},
};

impl State { 
    pub fn draw_parallax(&mut self, ctx: &mut Context) -> GameResult { 

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
                    .dest(render_pos(&self.parallax_info, &renderable.world_pos, &self.playerpos, SCREEN_MID_X))
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

    pub fn draw_splitscreen(&mut self, ctx: &mut Context) -> GameResult { 
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
                    .dest(render_pos(&self.parallax_info, &renderable.world_pos, &self.playerpos,  SCREEN_MID_X -SCREEN_QUART_X))
                    .offset([0.50, 0.91])
                    .scale([4.0, 4.0]));

                canvas2.draw(&*renderable.sprite, ggez::graphics::DrawParam::new()
                    .z((&renderable.world_pos.depth * -10.0) as i32)
                    .dest(render_pos_grid(&self.parallax_info, &renderable.world_pos, &self.playerpos, SCREEN_MID_X + SCREEN_QUART_X))
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

    pub fn adjust_parallax_linear(&mut self, ctx: &Context, adjustment: f32) { 
        self.parallax_info.parallax_top_y -= adjustment;
        let top_y = self.parallax_info.parallax_top_y;
        self.parallax_info.parallax_thickness_y += adjustment;
        let bot_y = top_y + self.parallax_info.parallax_thickness_y;

        self.parallax_info.splitscreen_parallax_mesh = build_splitscreen_parallax_mesh(&ctx, top_y, bot_y);
    }

    pub fn adjust_grid_sep_mult(&mut self, ctx: &Context, factor: f32) { 
        self.parallax_info.z_sep_top *= factor;
    }
}

pub fn build_parallax_info(ctx: &Context) -> ParallaxInfo { 
    ParallaxInfo {
        parallax_top_y: HORIZON,
        parallax_thickness_y: LAND_PROJECTION_HEIGHT,
        z_sep_top: Z_UNIT_TOP,
        is_splitscreen: true,
        splitscreen_back_mesh: build_splitscreen_main_mesh(&ctx),
        splitscreen_parallax_mesh: build_splitscreen_parallax_mesh(&ctx, HORIZON, HORIZON+LAND_PROJECTION_HEIGHT),
        back_mesh: build_back_mesh(&ctx),
        parallax_mesh: build_parallax_mesh(&ctx),
    }
}


#[allow(non_snake_case)]
pub fn render_pos(pxinf: &ParallaxInfo, world_pos: &WorldPos, playerx: &f32, midpoint: f32)->ggez::glam::Vec2 {
    let y = pxinf.parallax_top_y + (pxinf.parallax_thickness_y + Y_UNIT * world_pos.height) / (world_pos.depth * Z_UNIT + 1.0);
    let x = ((world_pos.x - playerx) * X_UNIT) / (world_pos.depth * Z_UNIT + 1.0) + midpoint;
    ggez::glam::Vec2::new(x, y)
}

const Z_UNIT_TOP: f32 = 32.0; // Separation degree for z in top-down view

#[allow(non_snake_case)]
pub fn render_pos_grid(pxinf: &ParallaxInfo, world_pos: &WorldPos, playerx: &f32, midpoint: f32)->ggez::glam::Vec2 {
    let y = Z_ORIGIN_Y_OFFSET - (world_pos.depth * pxinf.z_sep_top);
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

fn build_splitscreen_parallax_mesh(ctx: &Context, top_y: f32, bot_y: f32) -> GameResult<graphics::Mesh> {
    let mb: &mut graphics::MeshBuilder = &mut graphics::MeshBuilder::new();
    hor_line_half(mb, top_y, RED)?;
    hor_line_half(mb, bot_y, RED)?;
    Ok(graphics::Mesh::from_data(ctx, mb.build()))
}