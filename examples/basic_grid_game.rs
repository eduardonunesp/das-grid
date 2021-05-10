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
}

struct GameState {
    rect: Mesh,
    grid: Grid<i32>,
    player: Player,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // Create a grid with 20 rows and 10 cols with size of 32x32 pixels
        let mut grid = Grid::new((20, 10), (32., 32.), 0);

        // Create the player on pos 5,5
        let player = Player { pos: (5, 5) };

        // Set the player representation on grid
        grid.set(player.pos, &1)
            .expect("set value player at x = 5 and y = 5");

        // Rect used to represent grid cells
        let rect = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0., 0., SQR_RECT_SIZE, SQR_RECT_SIZE),
        )?;

        // Nice lets start
        Ok(Self { rect, grid, player })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        // Enumerate each cell position at the grid
        for (x, y) in self.grid.enumerate() {
            // Dest of each element to draw
            let dest = Vec2::new(
                128. + x as f32 * SQR_RECT_SIZE,
                64. + y as f32 * SQR_RECT_SIZE,
            );

            // Draw the grid
            self.rect.draw(ctx, dest);

            // Draw the player, player has id 1 the rest of the grid are 0 values
            if let Ok(v) = self.grid.get((x, y)) {
                // If Ok player will be found and we can draw it
                if v == &1 {
                    let mut dparam = DrawParams::new();
                    dparam.color = Color::RED;
                    dparam.position = dest;

                    // Draw the player
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

        // Press the direciton to apply the move on the player

        if input::is_key_pressed(ctx, Key::Right) {
            // If the move is ok it will move and return the next pos
            if let Ok(next_pos) = self
                .grid
                .mov_to(self.player.pos, das_grid::MoveDirection::Right)
            {
                // With the next pos we're going to update the player pos
                self.player.pos = next_pos;
            }
        }

        if input::is_key_pressed(ctx, Key::Left) {
            // If the move is ok it will move and return the next pos
            if let Ok(next_pos) = self
                .grid
                .mov_to(self.player.pos, das_grid::MoveDirection::Left)
            {
                // With the next pos we're going to update the player pos
                self.player.pos = next_pos;
            }
        }

        if input::is_key_pressed(ctx, Key::Up) {
            // If the move is ok it will move and return the next pos
            if let Ok(next_pos) = self
                .grid
                .mov_to(self.player.pos, das_grid::MoveDirection::Up)
            {
                // With the next pos we're going to update the player pos
                self.player.pos = next_pos;
            }
        }

        if input::is_key_pressed(ctx, Key::Down) {
            // If the move is ok it will move and return the next pos
            if let Ok(next_pos) = self
                .grid
                .mov_to(self.player.pos, das_grid::MoveDirection::Down)
            {
                // With the next pos we're going to update the player pos
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
