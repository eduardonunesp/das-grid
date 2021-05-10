use tetra::{
    graphics::{
        self,
        mesh::{Mesh, ShapeStyle},
        Color, Rectangle,
    },
    input::{self, Key},
    math::Vec2,
    window, Context, ContextBuilder, State,
};

use das_grid::Grid;

const SQR_RECT_SIZE: f32 = 32.;

struct GameState {
    rect: Mesh,
    grid: Grid<i32>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(Self {
            rect: Mesh::rectangle(
                ctx,
                ShapeStyle::Stroke(1.0),
                Rectangle::new(0., 0., SQR_RECT_SIZE, SQR_RECT_SIZE),
            )?,
            grid: Grid::new((20, 10), (32., 32.), 0),
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::BLACK);

        for (x, y) in self.grid.enumerate_to_cell_size() {
            let dest = Vec2::new(128. + x as f32, 64. + y as f32);
            self.rect.draw(ctx, dest);
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::Escape) {
            window::quit(ctx);
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Basic Grid Game", 600, 800)
        .build()?
        .run(GameState::new)
}
