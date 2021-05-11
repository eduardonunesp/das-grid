/*!
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
let mut g = das_grid::Grid::new((10, 10), (1., 1.), 0);

// Set the the value 1 at position x: 5 and y: 5
g.set((5, 5), &1);
```

### Bring your own type

```rust
// Using &str instead of i32
let mut g: das_grid::Grid<&str> = das_grid::Grid::new((10, 10), (1., 1.), "a");
g.get((0, 0)).unwrap(); // ouputs: "a"
```

```rust
// Your own enum, much better to track grid values
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pawn {
    None,
    Player,
    Enemy,
}

impl std::fmt::Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Pawn::None => write!(f, "None"),
            Pawn::Player => write!(f, "Player"),
            Pawn::Enemy => write!(f, "Enemy"),
        }
    }
}

// Initialize empty grid
let mut g: das_grid::Grid<Pawn> = das_grid::Grid::new((10, 10), (1., 1.), Pawn::None);

// Set the Player on position 5,5
g.set((5, 5), &Pawn::Player);

// Move the player to right
if let Ok((x, y)) = g.mov_to((5, 5), das_grid::MoveDirection::Right) {
    // "The pawn on 6,5 is Player"
    println!("The pawn on 6,5 is {}", g.get((x, y)).unwrap());
}
```

> The `mov_to` function can returns `Result<(), Err>` if the attept of move is out of the bounds of the grid

### Moving cells

Each tile of the grid is called cell and each cell is the type that you want, because it is a 2D structure each cell has an address which consists of X and Y

```rust
// Creates a 5x5 grid with 0 as default value for each cell
let mut g = das_grid::Grid::new((5, 5), (1., 1.), 0);

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
*/

#![allow(warnings, unused)]

mod grid;
pub use grid::*;

use std::{
    fmt::{self, Display},
    ops::{Index, IndexMut},
    result,
};

use parse_display_derive::Display;

/// A specialized [`Result`](std::result::Result) type for DasGrid.
pub type Result<T = ()> = result::Result<T, DasGridError>;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum DasGridError {
    #[display("value is out of the grid rows and cols")]
    OutOfGrid,
    #[display("failed to meet the rule requirements")]
    RuleFailed,
    #[display("the subgrid cols or rows is greater than the parent grid")]
    SubgridOverflow,
    #[display("the value isn't found at the position")]
    ValueNotFound,
}
