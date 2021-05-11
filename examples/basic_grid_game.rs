#![allow(warnings, unused)]

use tetra::{
    graphics::{
        self,
        mesh::{Mesh, ShapeStyle},
        Color, DrawParams, Rectangle,
    },
    input::{self, Key},
    math::Vec2,
    window, Context, ContextBuilder, State,
};

use das_grid::Grid;

struct GameState {}

impl GameState {
    pub fn new(_: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {})
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Basic Grid Game", 600, 800)
        .build()?
        .run(GameState::new)
}
