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
        let mut grid = Grid::new((16, 20), (32., 32.), 0);
        let (cell_w, cell_h) = grid.get_cell_size();

        // Create the player on pos 5,5
        let player = Player { pos: (5, 5) };

        // Set the player representation on grid
        grid.set(player.pos, &1)
            .expect("set value player at x = 5 and y = 5");

        // Rect used to represent grid cells
        let rect = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0., 0., cell_w, cell_h),
        )?;

        // Nice lets start
        Ok(Self { rect, grid, player })
    }

    fn draw_grid(&mut self, ctx: &mut Context) {
        let (cell_w, cell_h) = self.grid.get_cell_size();

        // Enumerate each cell position at the grid
        for (x, y, v) in self.grid.enumerate_with_valuef() {
            // Dest of each element to draw
            let dest = Vec2::new(10. + x * cell_w, 10. + y * cell_h);

            // Draw the grid
            self.rect.draw(ctx, dest);

            // If type is 1 should draw the player
            if v == &1 {
                let mut dparam = DrawParams::new();
                dparam.color = Color::RED;
                dparam.position = dest;

                // Draw the player
                self.rect.draw(ctx, dparam);
            }
        }
    }

    fn handle_input(&mut self, ctx: &mut Context) {
        if input::is_key_down(ctx, Key::Escape) {
            window::quit(ctx);
        }

        // Press the direction to apply the move on the player

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
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        self.draw_grid(ctx);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.handle_input(ctx);
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Basic Grid Game", 800, 600)
        .build()?
        .run(GameState::new)
}
