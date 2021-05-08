# Das Grid

**Das Grid** is a 2D grid library which serves as fundamental building block for any 2D game built on the concept of grid

Famous games built on 2d grid concept:

* Draughts/Checkers
* Chess
* Scrabble
* Tetris
* Bejeweled
* Shinning Force (while battle)

Das Grid offers:

* Generic grid type, you can use any type you want to be the grid cell
* Helpers to make easy the move of values inside the grid
* Based on 2D top/left to bottom/right concept (which can be updated in the future)

## Using **Das Grid**

### Creating the grid

```rust
// Creates a 10x10 grid with 0 as default value for each cell
let mut g = DasGrid::new(10, 10, 0);

// Set the the value 1 at position x: 5 and y: 5
g.set((5, 5), &1);
```

### Bring your own type

```rust
// Using &str instead of i32
let mut g: Grid<&str> = DasGrid::new(10, 10, "a");
println!(g.get((0, 0)).unwrap()); // ouputs: "a"
```

```rust
// Your own enum, much better to track grid values
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pawn {
    None,
    Player,
    Enemy,
}

impl fmt::Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pawn::None => write!(f, "None"),
            Pawn::Player => write!(f, "Player"),
            Pawn::Enemy => write!(f, "Enemy"),
        }
    }
}

// Initialize empty grid
let mut g: Grid<Pawn> = DasGrid::new(10, 10, Pawn::None);

// Set the Player on position 5,5
g.set((5, 5), &Pawn::Player);

// Move the player to right
if let Ok(()) = g.mov_to((5, 5), crate::MoveDirection::Right) {
    // "The pawn on 6,5 is Player"
    println!("The pawn on 6,5 is {}", g.get((6, 5)).unwrap());
}

```

> The `mov_to` function can returns `Result<(), OutOfGridErr>` if the attept of move is out of the bounds of the grid

### Moving cells

Each tile of the grid is called cell and each cell is the type that you want, because it is a 2D structure each cell has an address which consists of X and Y

```rust
// Creates a 5x5 grid with 0 as default value for each cell
let mut g = DasGrid::new(5, 5, 0);

// Print with special {:?} to see the contents of the grid
println!("{:?}", g);
// outputs:
// Grid { width: 5, height: 5, cells: [
//  0 (x: 0 y: 0) 0 (x: 1 y: 0) 0 (x: 2 y: 0) 0 (x: 3 y: 0) 0 (x: 4 y: 0)
//  0 (x: 0 y: 1) 0 (x: 1 y: 1) 0 (x: 2 y: 1) 0 (x: 3 y: 1) 0 (x: 4 y: 1)
//  0 (x: 0 y: 2) 0 (x: 1 y: 2) 0 (x: 2 y: 2) 0 (x: 3 y: 2) 0 (x: 4 y: 2)
//  0 (x: 0 y: 3) 0 (x: 1 y: 3) 0 (x: 2 y: 3) 0 (x: 3 y: 3) 0 (x: 4 y: 3)
//  0 (x: 0 y: 4) 0 (x: 1 y: 4) 0 (x: 2 y: 4) 0 (x: 3 y: 4) 0 (x: 4 y: 4)
// ] }
```
