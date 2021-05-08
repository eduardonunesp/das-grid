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
use std::fmt::Display;

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
let mut g: das_grid::Grid<Pawn> = das_grid::Grid::new(10, 10, Pawn::None);

// Set the Player on position 5,5
g.set((5, 5), &Pawn::Player);

// Move the player to right
if let Ok(()) = g.mov_to((5, 5), das_grid::MoveDirection::Right) {
    // "The pawn on 6,5 is Player"
    println!("The pawn on 6,5 is {}", g.get((6, 5)).unwrap());
}

```

> The `mov_to` function can returns `Result<(), Err>` if the attept of move is out of the bounds of the grid

### Moving cells

Each tile of the grid is called cell and each cell is the type that you want, because it is a 2D structure each cell has an address which consists of X and Y

```rust
// Creates a 5x5 grid with 0 as default value for each cell
let mut g = das_grid::Grid::new(5, 5, 0);

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

use std::{
    fmt::{self, Display},
    ops::{Index, IndexMut},
};

/// Err represents the errors that can happen on the Das Grid module
///
/// GridErr::OutOfGrid represent the error when the attempt of move or set a value
/// is beyond the bounds of grid
///
/// Err::RuleFailed represents the error when some rule failed to applied
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridErr {
    OutOfGrid,
    RuleFailed,
}

impl fmt::Display for GridErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GridErr::OutOfGrid => write!(f, "value is out of the grid width and height"),
            GridErr::RuleFailed => write!(f, "failed to meet the rule requirements"),
        }
    }
}

/// Represents the possible direction to move
///
/// MoveDirection::Right (1, 0)
/// MoveDirection::Left (-1, 0)
/// MoveDirection::Up (0, -1)
/// MoveDirection::Down (0, 1)
#[derive(Debug, PartialEq, Eq)]
pub enum MoveDirection {
    Right,
    Left,
    Up,
    Down,
}

/// Represent move to right position on Das Grid (1, 0)
pub const MOVE_RIGHT: (i32, i32) = (1, 0);

/// Represent move to left position on Das Grid (-1, 0)
pub const MOVE_LEFT: (i32, i32) = (-1, 0);

/// Represent move to up position on Das Grid (0, -1)
pub const MOVE_UP: (i32, i32) = (0, -1);

/// Represent move to down position on Das Grid (0, 1)
pub const MOVE_DOWN: (i32, i32) = (0, 1);

/// Stores the grid values and the cells
/// The grid itself representation is a flatten vector which is transformed
/// for 2D representation when called by the user
///
/// The cells are internally manage by a `Vec<T>`
///
/// So to create a grid with 4x4 (collums and rows)
///
/// ```.rust
/// let grid = das_grid::Grid::new(4, 4, 0);
/// assert_eq!(grid.size(), 16);
/// ```
///
/// Or if you like let's say a Tetris style grid
///
/// ```.rust
/// let grid = das_grid::Grid::new(10, 20, 0);
///
/// // And it will have 200 cells!
/// assert_eq!(grid.size(), 200);
/// ```

pub struct Grid<T: Copy + Clone> {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) initial_value: T,
    pub(crate) cells: Vec<T>,
}

impl<T: Copy + Clone> Grid<T> {
    /// Creates a grid of size rows x columns with default value passed on the third parameter
    /// For example this will generate a 2x2 grid of value 1:
    /// ```.rust
    /// let grid = das_grid::Grid::new(2, 2, 1);
    /// assert_eq!(grid.size(), 4);
    /// ```
    pub fn new(width: i32, height: i32, value: T) -> Self
    where
        T: Clone + Copy + Display,
    {
        let initial_value = value;
        let cells = vec![value; (width * height) as usize];

        if cells.is_empty() {
            panic!("0x0 grid is forbidden")
        }

        Self {
            width,
            height,
            initial_value,
            cells,
        }
    }

    /// Internally checks if the index (x, y) is inside of the bounds of the grid
    fn check_grid_bounds(&self, index: (i32, i32)) -> Result<(), GridErr> {
        let (x, y) = index;

        if x < 0 || x >= self.width {
            return Err(GridErr::OutOfGrid);
        }

        if y < 0 || y >= self.height {
            return Err(GridErr::OutOfGrid);
        }

        Ok(())
    }

    /// Sets a given value to the position (x, y)
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of GridErr::OutOfGrid
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new(2, 2, 1);
    /// assert!(grid.set((0, 0), &1).is_ok());
    /// ```
    pub fn set(&mut self, index: (i32, i32), value: &T) -> Result<(), GridErr>
    where
        T: Copy,
    {
        let (x, y) = index;

        self.check_grid_bounds(index)?;

        if let Some(cell) = self.cells.get_mut((x + (y * self.width)) as usize) {
            *cell = *value;
        }

        Ok(())
    }

    /// Sets a given value to the position (x, y)
    /// Only if no rule return error
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new(2, 2, 0);
    /// assert!(grid.set((0, 1), &1).is_ok());
    ///
    /// let rule_not_1 = |_: (i32, i32), value: &i32| -> Result<(), das_grid::GridErr> {
    ///     if *value == 1 {
    ///         return Err(das_grid::GridErr::RuleFailed);
    ///     }
    ///     Ok(())
    /// };
    ///
    /// assert!(
    ///     grid.set_with_rules((0, 1), &1, vec![rule_not_1])
    ///         .err()
    ///         .unwrap()
    ///         == das_grid::GridErr::RuleFailed
    /// );
    /// ```
    pub fn set_with_rules<R>(
        &mut self,
        index: (i32, i32),
        value: &T,
        rules: Vec<R>,
    ) -> Result<(), GridErr>
    where
        R: Fn((i32, i32), &T) -> Result<(), GridErr>,
    {
        for rule in rules.iter() {
            rule(index, value)?;
        }
        self.set(index, value)?;
        Ok(())
    }

    /// Gets a give value to the position (x, y) as mutable
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of GridErr::OutOfGrid
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new(2, 2, 1);
    /// let mut v = grid.get_mut((0, 0)).expect("cannnot get pos at (0, 0)");
    /// *v = 50;
    /// assert_eq!(grid.get((0, 0)).unwrap_or(&0), &50);
    /// ```
    pub fn get_mut(&mut self, index: (i32, i32)) -> Result<&mut T, GridErr> {
        let (x, y) = index;

        self.check_grid_bounds(index)?;

        Ok(self.cells.get_mut((x + (y * self.width)) as usize).unwrap())
    }

    /// Gets a give value to the position (x, y)
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of GridErr::OutOfGrid
    ///
    /// ```.rust
    /// let grid = das_grid::Grid::new(2, 2, 1);
    /// let v = grid.get((0, 0));
    /// assert_eq!(v, Ok(&1));
    /// ```
    pub fn get(&self, index: (i32, i32)) -> Result<&T, GridErr> {
        let (x, y) = index;

        self.check_grid_bounds(index)?;

        Ok(self.cells.get((x + (y * self.width)) as usize).unwrap())
    }

    /// Moves a given value from position (x, y) to destiny position (x, y)
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of GridErr::OutOfGrid
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new(2, 2, 1);
    /// assert_eq!(grid.mov((0, 0), (1, 1)), Ok(()));
    /// ```
    pub fn mov(&mut self, index: (i32, i32), dest: (i32, i32)) -> Result<(), GridErr> {
        self.check_grid_bounds(index)?;
        self.check_grid_bounds(dest)?;
        let prev = *self.get_mut(index).unwrap();
        self.set(index, &self.initial_value.clone())?;
        self.set(dest, &prev)?;

        Ok(())
    }

    /// Moves a given value from position (x, y) to another position based on the direction
    ///
    /// The directions can be Left, Right, Top, Down:
    /// * DasGrid::MoveDirection::Left, translates to (-1, 0)
    /// * DasGrid::MoveDirection::Right, translates to (1, 0)
    /// * DasGrid::MoveDirection::Top, translates to (0, -1)
    /// * DasGrid::MoveDirection::Down, translates to (0, 1)
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of GridErr::OutOfGrid
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new(2, 2, 1);
    /// assert_eq!(grid.mov_to((0, 0), das_grid::MoveDirection::Right), Ok(()));
    /// ```
    pub fn mov_to(&mut self, index: (i32, i32), direction: MoveDirection) -> Result<(), GridErr> {
        let (x, y) = index;
        self.check_grid_bounds(index)?;

        let (xx, yy) = match direction {
            MoveDirection::Up => MOVE_UP,
            MoveDirection::Down => MOVE_DOWN,
            MoveDirection::Left => MOVE_LEFT,
            MoveDirection::Right => MOVE_RIGHT,
        };

        let dest = (x + xx, y + yy);
        self.check_grid_bounds(dest)?;

        let prev = *self.get_mut(index).unwrap();
        self.set(index, &self.initial_value.clone())?;
        self.set(dest, &prev)?;

        Ok(())
    }

    /// Get the size of grid based on cells length
    ///
    /// For instance a 10x10 grid will return the size of 100
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new(2, 2, 1);
    /// assert_eq!(grid.size(), 4);
    /// ```
    pub fn size(&self) -> usize {
        self.cells.len()
    }

    /// The width of the grid
    /// ```.rust
    /// let mut grid = das_grid::Grid::new(3, 2, 1);
    /// assert_eq!(grid.width(), 3);
    /// ```
    pub fn width(&self) -> i32 {
        self.width
    }

    /// The height of the grid
    /// ```.rust
    /// let mut grid = das_grid::Grid::new(3, 2, 1);
    /// assert_eq!(grid.height(), 2);
    /// ```
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Returns the grid as a tuple of (x, y)
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new(3, 2, 1);
    /// for (x, y) in grid.enumerate() {
    ///     println!("x {} y {}", x, y);
    /// }
    /// ```
    pub fn enumerate(&self) -> Vec<(i32, i32)> {
        let mut x = 0;
        let mut y = 0;
        self.cells
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                if idx as i32 % self.width() == 0 && idx > 1 {
                    x = 0;
                    y += 1;
                }
                let res = (x, y);
                x += 1;
                res
            })
            .collect::<Vec<_>>()
    }
}

impl<'a, T: Copy + Clone> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

impl<'a, T: Copy + Clone> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter_mut()
    }
}

impl<T: Copy + Clone> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Grid {{ width: {}, height: {}, cells: [...] }}",
            self.width, self.height
        )
    }
}

impl<T: Copy + Clone + Display> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cell_str = String::new();

        let mut pos = (0, 0);
        for (idx, cell) in self.cells.iter().enumerate() {
            if idx as i32 % self.width == 0 && idx > 0 {
                pos.0 = 0;
                pos.1 += 1;
                cell_str += "\n";
            }
            cell_str.push_str(&format!("\t{:3} (x: {} y: {})", cell, pos.0, pos.1));
            pos.0 += 1
        }

        write!(
            f,
            "Grid {{ width: {}, height: {}, cells: [\n{}\n] }}",
            self.width, self.height, cell_str,
        )
    }
}

impl<T: Copy + Clone> Index<(i32, i32)> for Grid<T> {
    type Output = T;
    fn index(&self, index: (i32, i32)) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: Copy + Clone> IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

#[cfg(test)]
mod lib_test;
