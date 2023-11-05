use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

use das_grid::Grid;

const SQR_RECT_SIZE: f32 = 32.;

struct GameState {
    grid: Grid<i32>,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        // Load/create resources such as images here.
        GameState {
            grid: Grid::new((10, 10), (SQR_RECT_SIZE, SQR_RECT_SIZE), 0),
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        for (x, y) in self.grid.enumerate_to_cell_size() {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect([x, y, SQR_RECT_SIZE, SQR_RECT_SIZE].into())
                    .color(Color::BLUE),
            );
        }

        canvas.finish(ctx)
    }
}
fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("bgg", "Basic Grid Game")
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = GameState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}
