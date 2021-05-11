extern crate das_grid;
use parse_display_derive::Display;

// Your own enum, much better to track grid values
#[derive(Clone, Copy, PartialEq, Eq, Debug, Display)]
enum Pawn {
    #[display("None")]
    None,
    #[display("Player")]
    Player,
}

fn main() -> das_grid::Result {
    // Initialize empty grid
    let mut g: das_grid::Grid<Pawn> = das_grid::Grid::new((2, 2), (32., 32.), Pawn::None);

    // Set the Player on position 5,5
    g.set((0, 0), &Pawn::Player)?;

    println!("Initial state {:?}\n", g);

    // Move the player to right
    if let Ok(_) = g.mov_to((0, 0), das_grid::MoveDirection::Right) {
        // "The pawn on 5,6 is Player"
        println!("The pawn on 0, 1 is {}\n", g.get((0, 1)).unwrap());
    }

    println!("End state {:?}\n", g);

    Ok(())
}
