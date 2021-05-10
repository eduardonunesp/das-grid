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

const SQR_RECT_SIZE: f32 = 32.;

struct Player {
    pos: (i32, i32),
    id: i32,
}

struct GameState {
    rect: Mesh,
    grid: Grid<i32>,
    player: Player,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut grid = Grid::new((20, 10), (32., 32.), 0);

        grid.set((5, 5), &1).expect("set value 1 at 5,5");

        let player = Player { pos: (5, 5), id: 1 };

        Ok(Self {
            rect: Mesh::rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0., 0., SQR_RECT_SIZE, SQR_RECT_SIZE),
            )?,
            grid,
            player,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::BLACK);

        for (x, y) in self.grid.enumerate() {
            let dest = Vec2::new(
                128. + x as f32 * SQR_RECT_SIZE,
                64. + y as f32 * SQR_RECT_SIZE,
            );

            self.rect.draw(ctx, dest);

            if let Ok(v) = self.grid.get((x, y)) {
                if v == &1 {
                    let mut dparam = DrawParams::new();
                    dparam.color = Color::RED;
                    dparam.position = dest;

                    self.rect.draw(ctx, dparam);
                }
            }
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::Escape) {
            window::quit(ctx);
        }

        if input::is_key_pressed(ctx, Key::Right) {
            if let Ok(next_pos) = self
                .grid
                .mov_to(self.player.pos, das_grid::MoveDirection::Right)
            {
                self.player.pos = next_pos;
            }
        }

        if input::is_key_pressed(ctx, Key::Left) {
            if let Ok(next_pos) = self
                .grid
                .mov_to(self.player.pos, das_grid::MoveDirection::Left)
            {
                self.player.pos = next_pos;
            }
        }

        if input::is_key_pressed(ctx, Key::Up) {
            if let Ok(next_pos) = self
                .grid
                .mov_to(self.player.pos, das_grid::MoveDirection::Up)
            {
                self.player.pos = next_pos;
            }
        }

        if input::is_key_pressed(ctx, Key::Down) {
            if let Ok(next_pos) = self
                .grid
                .mov_to(self.player.pos, das_grid::MoveDirection::Down)
            {
                self.player.pos = next_pos;
            }
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Basic Grid Game", 600, 800)
        .build()?
        .run(GameState::new)
}
