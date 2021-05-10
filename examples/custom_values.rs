extern crate das_grid;

// Your own enum, much better to track grid values
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pawn {
    None,
    Player,
}

impl std::fmt::Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Pawn::None => write!(f, "None"),
            Pawn::Player => write!(f, "Player"),
        }
    }
}

fn main() -> Result<(), das_grid::GridErr> {
    // Initialize empty grid
    let mut g: das_grid::Grid<Pawn> = das_grid::Grid::new(2, 2, Pawn::None);

    // Set the Player on position 5,5
    g.set((0, 0), &Pawn::Player)?;

    println!("Initial state {:?}\n", g);

    // Move the player to right
    if let Ok(()) = g.mov_to((0, 0), das_grid::MoveDirection::Right) {
        // "The pawn on 5,6 is Player"
        println!("The pawn on 0, 1 is {}\n", g.get((0, 1)).unwrap());
    }

    println!("End state {:?}\n", g);

    Ok(())
}
