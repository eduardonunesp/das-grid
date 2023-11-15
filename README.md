# Das Grid

[![Test](https://github.com/eduardonunesp/das-grid/actions/workflows/test.yml/badge.svg)](https://github.com/eduardonunesp/das-grid/actions/workflows/test.yml)

**Das Grid** is a 2D grid library which serves as fundamental building block for any 2D game built on the concept of grid

<img alt="Screenshot 2023-11-15 at 11 12 33" src="https://github.com/eduardonunesp/das-grid/assets/582516/859c2a73-8e4f-4a3a-b369-889a1d1ca29d">

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

Also, check the documentation at [docs.rs/das-grid](https://docs.rs/das-grid/0.1.5/das_grid/index.html)

### Creating the grid

```rust
// Creates a 10x10 grid with 0 as default value for each cell
let mut g = das_grid::Grid::new(10, 10, 0);

// Set the the value 1 at position x: 5 and y: 5
g.set((5, 5), &1);
```

### Bring your own type

```rust
// Using &str instead of i32
let mut g: das_grid::Grid<&str> = das_grid::Grid::new(10, 10, "a");
g.get((0, 0)).unwrap(); // ouputs: "a"
```

```rust
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
    let mut g: das_grid::Grid<Pawn> = das_grid::Grid::new(8, 8, Pawn::None);

    // Set the Player on position 5,5
    g.set((5, 5), &Pawn::Player)?;

    println!("Grid initial state {:?}", g);

    // Move the player to right
    if let Ok(()) = g.mov_to((5, 5), das_grid::MoveDirection::Right) {
        // "The pawn on 5,6 is Player"
        println!("The pawn on 5,6 is {}", g.get((6, 5)).unwrap());
    }

    println!("Grid initial end state {:?}", g);

    Ok(())
}
```

> The `mov_to` function can returns `Result<(), OutOfGridErr>` if the attept of move is out of the bounds of the grid

## Example

To build and run an example:

```bash
cargo run --example custom_values

# Output

# Initial state Grid { rows: 2, cols: 2, cells: [
#  Player (x: 0 y: 0) None (x: 0 y: 1)
#  None   (x: 1 y: 0) None (x: 1 y: 1)
# ] }
#
# The pawn on 0, 1 is Player
#
# End state Grid { rows: 2, cols: 2, cells: [
#  None (x: 0 y: 0) Player (x: 0 y: 1)
#  None (x: 1 y: 0) None   (x: 1 y: 1)
# ] }
```

### Moving cells

Each tile of the grid is called cell and each cell is the type that you want, because it is a 2D structure each cell has an address which consists of X and Y

```rust
// Creates a 5x5 grid with 0 as default value for each cell
let mut g = das_grid::Grid::new(5, 5, 0);

// Print with special {:?} to see the contents of the grid
println!("{:?}", g);
// outputs:
// Grid { rows: 5, cols: 5, cells: [
//  0 (x: 0 y: 0) 0 (x: 1 y: 0) 0 (x: 2 y: 0) 0 (x: 3 y: 0) 0 (x: 4 y: 0)
//  0 (x: 0 y: 1) 0 (x: 1 y: 1) 0 (x: 2 y: 1) 0 (x: 3 y: 1) 0 (x: 4 y: 1)
//  0 (x: 0 y: 2) 0 (x: 1 y: 2) 0 (x: 2 y: 2) 0 (x: 3 y: 2) 0 (x: 4 y: 2)
//  0 (x: 0 y: 3) 0 (x: 1 y: 3) 0 (x: 2 y: 3) 0 (x: 3 y: 3) 0 (x: 4 y: 3)
//  0 (x: 0 y: 4) 0 (x: 1 y: 4) 0 (x: 2 y: 4) 0 (x: 3 y: 4) 0 (x: 4 y: 4)
// ] }
```

As explained before to move cells you can call the function `mov_to` and pass the origin and destiny as a direction

The directions can be Left, Right, Top, Down:

* DasGrid::MoveDirection::Left, translates to (-1, 0)
* DasGrid::MoveDirection::Right, translates to (1, 0)
* DasGrid::MoveDirection::Top, translates to (0, -1)
* DasGrid::MoveDirection::Down, translates to (0, 1)

> The `mov_to` function can returns `Result<(), OutOfGridErr>` if the attept of move is out of the bounds of the grid

### Iterators

The grid has implemented few iterators that can be very handy on daily usage:

#### Iterating over the flatten grid structure

```rust
let grid: DasGrid<i32> = DasGrid::new(2, 2, 0);
let mut result: Vec<i32> = vec![];
for v in &grid {
    println!("Value {}", value);
}
assert!(result == [0, 0, 0, 0]);
```

#### Iterating over the enumerate of X and Y

```rust
let mut grid: DasGrid<i32> = DasGrid::new(2, 2, 0);
// Returns the X and Y as tuple
for (x, y) in grid.enumerate() {
    println!("X {} Y {}", x, y);
}
```

## License

```text
MIT License

Copyright (c) 2021 Eduardo Pereira

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
