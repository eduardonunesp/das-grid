use parse_display_derive::Display;
use std::{
    fmt::{self, Display},
    ops::{Index, IndexMut},
};

use crate::{DasGridError, Result};

/// Represents the possible direction to move
///
/// MoveDirection::Right
/// MoveDirection::Left
/// MoveDirection::Up
/// MoveDirection::Down
#[derive(Debug, PartialEq, Eq, Display)]
pub enum MoveDirection {
    #[display("Right (1, 0)")]
    /// (1, 0)
    Right,
    #[display("Left (-1, 0)")]
    /// (-1, 0)
    Left,
    #[display("Up (0, -1")]
    /// (0, -1)
    Up,
    #[display("Down (0, 1)")]
    /// (0, 1)
    Down,
}

/// Represent move to right position on Das Grid (0, 1)
pub const MOVE_RIGHT: (i32, i32) = (1, 0);

/// Represent move to left position on Das Grid (0, -1)
pub const MOVE_LEFT: (i32, i32) = (-1, 0);

/// Represent move to up position on Das Grid (-1, 0)
pub const MOVE_UP: (i32, i32) = (0, -1);

/// Represent move to down position on Das Grid (1, 0)
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
/// let grid = das_grid::Grid::new((4, 4), (1., 1.), 0);
/// assert_eq!(grid.size(), 16);
/// ```
///
/// Or if you like let's say a Tetris style grid
///
/// ```.rust
/// let grid = das_grid::Grid::new((20, 10), (32., 32.), 0);
///
/// // And it will have 200 cells!
/// assert_eq!(grid.size(), 200);
/// ```
pub struct Grid<T: Copy> {
    pub(crate) frame_size: (i32, i32),
    pub(crate) cell_size: (f32, f32),
    pub(crate) default_value: T,
    pub(crate) cells: Vec<T>,
}

pub struct FrameSizeParams {
    rows: i32,
    cols: i32,
}

impl From<(i32, i32)> for FrameSizeParams {
    fn from(frame_size: (i32, i32)) -> Self {
        let (rows, cols) = frame_size;
        Self { rows, cols }
    }
}

pub struct CellSizeParams {
    width: f32,
    height: f32,
}

impl From<(f32, f32)> for CellSizeParams {
    fn from(cell_size: (f32, f32)) -> Self {
        let (width, height) = cell_size;
        Self { width, height }
    }
}

impl<T: Copy> Grid<T> {
    /// Creates a grid of size rows x columns with default value passed on the third parameter
    /// For example this will generate a 2x2 grid of value 1:
    /// ```.rust
    /// let grid = das_grid::Grid::new((2, 2),(1., 1.), 1);
    /// assert_eq!(grid.size(), 4);
    /// ```
    pub fn new<F, C>(frame_size: F, cell_size: C, default_value: T) -> Self
    where
        F: Into<FrameSizeParams>,
        C: Into<CellSizeParams>,
    {
        let frame_size = frame_size.into();
        let cell_size = cell_size.into();

        if (frame_size.rows * frame_size.cols) == 0 {
            panic!("0x0 grid is forbidden")
        }

        let cells = vec![default_value; (frame_size.rows * frame_size.cols) as usize];

        Self {
            frame_size: (frame_size.rows, frame_size.cols),
            cell_size: (cell_size.width, cell_size.height),
            default_value,
            cells,
        }
    }

    /// Creates a grid from a given vector with quadratic size
    /// For example this will generate a 2x2 grid
    /// ```.rust
    /// let mut grid = das_grid::Grid::new_from_vector((2, 2), (1.,1.), vec![1, 2, 3, 4]);
    /// assert_eq!(grid.size(), 4);
    /// ```
    pub fn new_from_vector<F, C>(frame_size: F, cell_size: C, vec: Vec<T>) -> Self
    where
        F: Into<FrameSizeParams>,
        C: Into<CellSizeParams>,
    {
        let frame_size = frame_size.into();
        let cell_size = cell_size.into();

        if vec.len() % 2 != 0 {
            panic!("The vector isn't multiple of 2");
        }

        if vec.len() == 0 {
            panic!("0x0 grid is forbidden")
        }

        if frame_size.rows * frame_size.cols != vec.len() as i32 {
            panic!("cols and rows should be same vector size")
        }

        let default_value = vec.first().unwrap().clone();
        let cells = vec.to_vec();

        Self {
            frame_size: (frame_size.rows, frame_size.cols),
            cell_size: (cell_size.width, cell_size.height),
            default_value,
            cells,
        }
    }

    // Check if subgrid isn't bigger than the destiny grid
    fn check_grid_overflow(&self, sub_grid: &Grid<T>) -> Result {
        let (rows, cols) = self.frame_size;
        let (sg_rows, sg_cols) = sub_grid.frame_size;

        if sg_cols > cols {
            return Err(DasGridError::SubgridOverflow);
        }

        if sg_rows > rows {
            return Err(DasGridError::SubgridOverflow);
        }

        Ok(())
    }

    /// Internally checks if the index (x, y) is inside of the bounds of the grid
    fn check_grid_bounds(&self, dst: (i32, i32)) -> Result {
        let (rows, cols) = self.frame_size;
        let (x, y) = dst;

        if x < 0 || x >= cols {
            return Err(DasGridError::OutOfGrid);
        }

        if y < 0 || y >= rows {
            return Err(DasGridError::OutOfGrid);
        }

        Ok(())
    }
}

impl<T: Copy> Grid<T> {
    /// Stamps the subgrid into the destiny grid, merging both
    ///
    /// If the sub grid is greater than the main grid it return an error of GridErr::SubgridOverflow
    /// Or if the dest x, y grid is out of bounds it return error GridErr::OutOfGrid
    ///
    /// ```.rust
    /// let mut grid: das_grid::Grid<i32> = das_grid::Grid::new((10, 10), (1., 1.), 0);
    /// let sub_grid: das_grid::Grid<i32> = das_grid::Grid::new((2, 2),(1., 1.), 1);
    /// assert!(grid.stamp_subgrid((5, 5), sub_grid).is_ok());
    /// assert_eq!(grid.get((5, 5)).unwrap(), &1);
    /// assert_eq!(grid.get((5, 6)).unwrap(), &1);
    /// assert_eq!(grid.get((6, 5)).unwrap(), &1);
    /// assert_eq!(grid.get((6, 6)).unwrap(), &1);
    /// ```
    pub fn stamp_subgrid(&mut self, dst: (i32, i32), sub_grid: &Grid<T>) -> Result {
        self.check_grid_overflow(&sub_grid)?;
        self.check_grid_bounds(dst)?;
        let (xx, yy) = dst;

        for (x, y, _) in sub_grid.enumerate_with_value() {
            if let Ok(subv) = sub_grid.get((x, y)) {
                // Sum origin of subgrid and dest cells
                let dest = (yy + y, xx + x);

                // Ok if the subgrid bleeds
                match self.set(dest, &subv) {
                    Ok(_) => (),
                    _ => (),
                }
            }
        }

        Ok(())
    }

    /// Stamps the subgrid into the destiny grid, merging both
    /// Only if no rule return error
    ///
    /// If the sub grid is greater than the main grid it return an error of DasGridError::SubgridOverflow
    ///
    /// Or if the dest x, y grid is out of bounds it return error DasGridError::OutOfGrid
    ///
    /// And if a rule some rule failed it will return DasGridError::RuleFailed
    ///
    /// ```.rust
    /// let mut grid: das_grid::Grid<i32> = das_grid::Grid::new((10, 10), (1., 1.), 1);
    /// let sub_grid: das_grid::Grid<i32> = das_grid::Grid::new((2, 2),(1., 1.), 1);
    ///
    /// let rule_not_1 = |_: (i32, i32), value: &i32| -> Result<(), das_grid::DasGridError> {
    ///     if *value == 1 {
    ///         return Err(das_grid::DasGridError::RuleFailed);
    ///     }
    ///     Ok(())
    /// };
    ///
    /// assert!(grid
    ///     .stamp_subgrid_with_rules((5, 5), sub_grid, vec![rule_not_1])
    ///     .is_err());
    /// ```
    pub fn stamp_subgrid_with_rules<R>(
        &mut self,
        dst: (i32, i32),
        sub_grid: Grid<T>,
        rules: Vec<R>,
    ) -> Result
    where
        R: Fn((i32, i32), &T) -> Result,
    {
        self.check_grid_overflow(&sub_grid)?;
        self.check_grid_bounds(dst)?;
        let (xx, yy) = dst;

        for (x, y, _) in sub_grid.enumerate_with_value() {
            if let Ok(subv) = sub_grid.get((x, y)) {
                // Sum origin of subgrid and dest cells
                let dest = (xx + x, yy + y);

                // Get the destiny
                let destv = self.get(dest)?;

                // Test rules on dest pos and value
                for rule in rules.iter() {
                    rule(dest, destv)?;
                }

                // Ok if the subgrid bleeds
                match self.set(dest, &subv) {
                    Ok(_) => (),
                    _ => (),
                }
            }
        }

        Ok(())
    }

    /// Sets a given value to the position (x, y)
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of GridErr::OutOfGrid
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((2, 2),(1., 1.), 1);
    /// assert!(grid.set((0, 0), &1).is_ok());
    /// ```
    pub fn set(&mut self, dst: (i32, i32), value: &T) -> Result {
        let (rows, _) = self.frame_size;
        let (x, y) = dst;

        self.check_grid_bounds(dst)?;

        if let Some(cell) = self.cells.get_mut((x * rows + y) as usize) {
            *cell = *value;
        }

        Ok(())
    }

    /// Sets a given value to the position (x, y)
    /// Only if no rule return error
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((2, 2),(1., 1.), 0);
    /// assert!(grid.set((0, 1), &1).is_ok());
    ///
    /// let rule_not_1 = |_: (i32, i32), value: &i32| -> Result<(), das_grid::DasGridError> {
    ///     if *value == 1 {
    ///         return Err(das_grid::DasGridError::RuleFailed);
    ///     }
    ///     Ok(())
    /// };
    ///
    /// assert!(
    ///     grid.set_with_rules((0, 1), &1, vec![rule_not_1])
    ///         .err()
    ///         .unwrap()
    ///         == das_grid::DasGridError::RuleFailed
    /// );
    /// ```
    pub fn set_with_rules<R>(&mut self, dst: (i32, i32), value: &T, rules: Vec<R>) -> Result
    where
        R: Fn((i32, i32), &T) -> Result,
    {
        for rule in rules.iter() {
            rule(dst, value)?;
        }
        self.set(dst, value)?;
        Ok(())
    }

    /// Moves a given value from position (x, y) to destiny position (x, y)
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of DasGridError::OutOfGrid
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((2, 2),(1., 1.), 1);
    /// assert_eq!(grid.mov((0, 0), (1, 1)), Ok(()));
    /// ```
    pub fn mov(&mut self, src: (i32, i32), dest: (i32, i32)) -> Result {
        self.check_grid_bounds(src)?;
        self.check_grid_bounds(dest)?;
        let prev = *self.get_mut(src).unwrap();
        self.set(src, &self.default_value.clone())?;
        self.set(dest, &prev)?;

        Ok(())
    }

    /// Moves a given value from position (x, y) to destiny position (x, y)
    /// Only if no rule return error
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of DasGridError::OutOfGrid
    ///
    /// And if a rule some rule failed it will return DasGridError::RuleFailed
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((2, 2),(1., 1.), 0);
    /// assert!(grid.set((0, 1), &1).is_ok());
    ///
    /// let rule_not_1 = |_: (i32, i32), value: &i32| -> Result<(), das_grid::DasGridError> {
    ///     if *value == 1 {
    ///         return Err(das_grid::DasGridError::RuleFailed);
    ///     }
    ///     Ok(())
    /// };
    ///
    /// assert!(
    ///     grid.mov_with_rules((0, 0), (0, 1), vec![rule_not_1])
    ///         .err()
    ///         .unwrap()
    ///         == das_grid::DasGridError::RuleFailed
    /// );
    /// ```
    pub fn mov_with_rules<R>(&mut self, src: (i32, i32), dst: (i32, i32), rules: Vec<R>) -> Result
    where
        R: Fn((i32, i32), &T) -> Result,
    {
        self.check_grid_bounds(src)?;
        self.check_grid_bounds(dst)?;
        let prev = *self.get_mut(src).unwrap();

        let destv = self.get(dst)?;
        for rule in rules {
            rule(dst, destv)?;
        }

        self.set(src, &self.default_value.clone())?;
        self.set(dst, &prev)?;

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
    /// let mut grid = das_grid::Grid::new((2, 2),(1., 1.), 1);
    /// assert_eq!(grid.mov_to((0, 0), das_grid::MoveDirection::Down), Ok((0, 1)));
    /// ```
    pub fn mov_to(&mut self, src: (i32, i32), dst_direction: MoveDirection) -> Result<(i32, i32)> {
        let (x, y) = src;
        self.check_grid_bounds(src)?;

        let (xx, yy) = match dst_direction {
            MoveDirection::Up => MOVE_UP,
            MoveDirection::Down => MOVE_DOWN,
            MoveDirection::Left => MOVE_LEFT,
            MoveDirection::Right => MOVE_RIGHT,
        };

        let dest = (x + xx, y + yy);
        self.check_grid_bounds(dest)?;

        let prev = *self.get_mut(src)?;
        self.set(src, &self.default_value.clone())?;
        self.set(dest, &prev)?;

        Ok(dest)
    }

    /// Fills the certain area of the grid with a given value
    ///
    /// If the area is greater than the main grid it return an error of GridErr::SubgridOverflow
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new_from_vector((4, 4), (1., 1.), (1..=16).collect());
    /// grid.fill_subgrid((1, 1), (2, 2), &0);
    /// assert!(grid.get((1, 1)).unwrap() == &0);
    /// assert!(grid.get((1, 2)).unwrap() == &0);
    /// assert!(grid.get((2, 1)).unwrap() == &0);
    /// assert!(grid.get((2, 2)).unwrap() == &0);
    /// ```
    pub fn fill_subgrid(
        &mut self,
        dst: (i32, i32),
        frame_size: (i32, i32),
        value: &T,
    ) -> Result<Grid<T>> {
        self.check_grid_bounds(dst)?;
        let sub_grid = Grid::new(frame_size, self.cell_size, self.default_value);
        self.check_grid_overflow(&sub_grid)?;

        for sub_index in sub_grid.enumerate() {
            let dest = (dst.0 + sub_index.0, dst.1 + sub_index.1);
            match self.set(dest, value) {
                Ok(_) => (),
                _ => (),
            }
        }

        Ok(sub_grid)
    }

    /// Moves a given value from position (x, y) to another position based on the direction
    /// Only if no rule return error
    ///
    /// if the dest x, y grid is out of bounds it return error DasGridError::OutOfGrid
    ///
    /// And if a rule some rule failed it will return DasGridError::RuleFailed
    ///
    /// The directions can be Left, Right, Top, Down:
    /// * DasGrid::MoveDirection::Left, translates to (0, -1)
    /// * DasGrid::MoveDirection::Right, translates to (0, 1)
    /// * DasGrid::MoveDirection::Top, translates to (-1, 0)
    /// * DasGrid::MoveDirection::Down, translates to (1, 0)
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of DasGridError::OutOfGrid
    ///
    /// ```.rust
    /// let mut g = das_grid::Grid::new((2, 2),(1., 1.), 0);
    /// g.set((0, 1), &1);
    /// let rule_not_1 = |_: (i32, i32), value: &i32| -> Result<(), das_grid::DasGridError> {
    ///     if *value == 1 {
    ///         return Err(das_grid::DasGridError::RuleFailed);
    ///     }
    ///     Ok(())
    /// };
    /// let ret = g.mov_to_with_rules((0, 0), das_grid::MoveDirection::Down, vec![rule_not_1]);
    /// assert!(ret.is_err());
    /// ```
    pub fn mov_to_with_rules<R>(
        &mut self,
        src: (i32, i32),
        dst_direction: MoveDirection,
        rules: Vec<R>,
    ) -> Result<(i32, i32)>
    where
        R: Fn((i32, i32), &T) -> Result,
    {
        let (x, y) = src;
        self.check_grid_bounds(src)?;

        let (xx, yy) = match dst_direction {
            MoveDirection::Up => MOVE_UP,
            MoveDirection::Down => MOVE_DOWN,
            MoveDirection::Left => MOVE_LEFT,
            MoveDirection::Right => MOVE_RIGHT,
        };

        let dest = (x + xx, y + yy);
        self.check_grid_bounds(dest)?;

        let destv = self.get(dest)?;
        for rule in rules {
            rule(dest, destv)?;
        }

        let prev = *self.get_mut(src).unwrap();
        self.set(src, &self.default_value.clone())?;
        self.set(dest, &prev)?;

        Ok(dest)
    }

    /// Gets a give value to the position (x, y) as mutable
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of GridErr::OutOfGrid
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((2, 2),(1., 1.), 1);
    /// let mut v = grid.get_mut((0, 0)).expect("cannnot get pos at (0, 0)");
    /// *v = 50;
    /// assert_eq!(grid.get((0, 0)).unwrap_or(&0), &50);
    /// ```
    pub fn get_mut(&mut self, src: (i32, i32)) -> Result<&mut T> {
        let (rows, _) = self.frame_size;
        let (x, y) = src;

        self.check_grid_bounds(src)?;

        let v = self
            .cells
            .get_mut((x * rows + y) as usize)
            .ok_or(DasGridError::ValueNotFound)?;
        Ok(v)
    }
}

impl<T: Copy> Grid<T> {
    /// Gets a give value to the position (x, y)
    ///
    /// Be careful if the value is out of the bounds of grid it will return an error
    /// with the type of GridErr::OutOfGrid
    ///
    /// ```.rust
    /// let grid = das_grid::Grid::new((2, 2),(1., 1.), 1);
    /// let v = grid.get((0, 0));
    /// assert_eq!(v, Ok(&1));
    /// ```
    pub fn get(&self, src: (i32, i32)) -> Result<&T> {
        let (rows, _) = self.frame_size;
        let (x, y) = src;

        self.check_grid_bounds(src)?;

        let v = self
            .cells
            .get((x * rows + y) as usize)
            .ok_or(DasGridError::ValueNotFound)?;
        Ok(v)
    }

    /// Returns the type vector with the values from the col
    ///
    /// If the col idx is wrong it can return the error DasGridError::OutOfGrid
    ///
    /// ```.rust
    /// let mut g = das_grid::Grid::new_from_vector((2, 2), (1.,1.), vec![1, 2, 3, 4]);
    /// let col = g.get_col(1).unwrap();
    /// assert_eq!(col, vec![2, 4]);
    /// ```
    pub fn get_col(&self, col_idx: i32) -> Result<Vec<T>> {
        let (_, cols) = self.frame_size;
        let mut vec_result: Vec<T> = vec![];
        for idx in (0..cols).into_iter() {
            let v = self.get((idx, col_idx))?;
            vec_result.push(*v);
        }
        Ok(vec_result)
    }

    /// Returns the type vector with the values from the row
    ///
    /// If the row idx is wrong it can return the error DasGridError::OutOfGrid
    ///
    /// ```.rust
    /// let mut g = das_grid::Grid::new_from_vector((2, 2), (1.,1.), vec![1, 2, 3, 4]);
    /// let row = g.get_row(1).unwrap();
    /// assert_eq!(row, vec![3, 4]);
    /// ```
    pub fn get_row(&self, row_idx: i32) -> Result<Vec<T>> {
        let (rows, _) = self.frame_size;
        let mut vec_result: Vec<T> = vec![];
        for idx in (0..rows).into_iter() {
            let v = self.get((row_idx, idx))?;
            vec_result.push(*v);
        }
        Ok(vec_result)
    }

    /// Creates the a new grid which is a snapshot of the main grid on the given position and size
    ///
    /// If the sub grid is greater than the main grid it return an error of GridErr::SubgridOverflow
    ///
    /// Or if the dest x, y grid is out of bounds it return error GridErr::OutOfGrid
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new_from_vector((4, 4), (1., 1.), (1..=16).collect());
    /// let sub_grid = grid.get_subgrid((2, 2), 2, 2).unwrap();
    /// assert_eq!(sub_grid.get_flatten_grid(), vec![11, 12, 15, 16]);
    /// ```
    pub fn get_subgrid(&self, src: (i32, i32), rows: i32, cols: i32) -> Result<Grid<T>> {
        self.check_grid_bounds(src)?;
        let mut sub_grid = Grid::new((rows, cols), self.cell_size, self.default_value);
        self.check_grid_overflow(&sub_grid)?;

        let mut x = 0;
        let mut y = 0;

        for (x, y) in sub_grid.enumerate() {
            let dest = (src.0 + x, src.1 + y);
            if let Ok(subv) = self.get(dest) {
                match sub_grid.set((x, y), &subv) {
                    Ok(_) => (),
                    _ => (),
                }
            }
        }

        Ok(sub_grid)
    }

    /// Returns a clone of the internal representation of the grid
    ///
    /// ```.rust
    /// let mut g = das_grid::Grid::new_from_vector((2, 2), (1.,1.), vec![1, 2, 3, 4]);
    /// assert_eq!(g.get_flatten_grid(), vec![1,2,3,4]);
    /// ```
    pub fn get_flatten_grid(&self) -> Vec<T> {
        self.cells.clone()
    }

    /// Returns the grid as a tuple of (x, y, v) as integer
    /// x, y as positions and v as the value of the cell
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((3, 2), (1., 1.), 1);
    /// for (x, y, v) in grid.enumerate_with_value() {
    ///     println!("x {} y {} v {}", x, y, v);
    /// }
    /// ```
    pub fn enumerate_with_value(&self) -> Vec<(i32, i32, &T)> {
        let mut x = 0;
        let mut y = 0;
        let (rows, _) = self.frame_size;
        self.cells
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                if idx as i32 % rows == 0 && idx > 1 {
                    y = 0;
                    x += 1;
                }
                let res = (x, y, v);
                y += 1;
                res
            })
            .collect::<Vec<_>>()
    }

    /// Returns the grid as a tuple of (x, y, v) as float
    /// x, y as positions and v as the value of the cell
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((3, 2), (1., 1.), 1);
    /// for (x, y, v) in grid.enumerate_with_valuef() {
    ///     println!("x {} y {} v {}", x, y, v);
    /// }
    /// ```
    pub fn enumerate_with_valuef(&self) -> Vec<(f32, f32, &T)> {
        let mut x = 0;
        let mut y = 0;
        let (rows, _) = self.frame_size;
        self.cells
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                if idx as i32 % rows == 0 && idx > 1 {
                    y = 0;
                    x += 1;
                }
                let res = (x as f32, y as f32, v);
                y += 1;
                res
            })
            .collect::<Vec<_>>()
    }

    /// Returns the grid as a tuple of (x, y) as integer
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((3, 2), (1., 1.), 1);
    /// for (x, y) in grid.enumerate() {
    ///     println!("x {} y {}", x, y);
    /// }
    /// ```
    pub fn enumerate(&self) -> Vec<(i32, i32)> {
        let mut x = 0;
        let mut y = 0;
        let (rows, _) = self.frame_size;
        self.cells
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                if idx as i32 % rows == 0 && idx > 1 {
                    y = 0;
                    x += 1;
                }
                let res = (x, y);
                y += 1;
                res
            })
            .collect::<Vec<_>>()
    }

    /// Returns the grid as a tuple of (x, y) as float
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((3, 2), (1., 1.), 1);
    /// for (x, y) in grid.enumerate() {
    ///     println!("x {} y {}", x, y);
    /// }
    /// ```
    pub fn enumeratef(&self) -> Vec<(f32, f32)> {
        let mut x = 0;
        let mut y = 0;
        let (rows, _) = self.frame_size;
        self.cells
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                if idx as i32 % rows == 0 && idx > 1 {
                    y = 0;
                    x += 1;
                }
                let res = (x as f32, y as f32);
                y += 1;
                res
            })
            .collect::<Vec<_>>()
    }

    // Debug only internal
    pub(crate) fn debug(&self)
    where
        T: fmt::Display + Copy,
    {
        println!("{:?}", self)
    }

    /// Get the size of grid based on cells length
    ///
    /// For instance a 10x10 grid will return the size of 100
    ///
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((2, 2),(1., 1.), 1);
    /// assert_eq!(grid.size(), 4);
    /// ```
    pub fn size(&self) -> usize {
        self.cells.len()
    }

    /// The rows of the grid
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((3, 2), (1., 1.), 1);
    /// assert_eq!(grid.rows(), 3);
    /// ```
    pub fn rows(&self) -> i32 {
        self.frame_size.0
    }

    /// The cols of the grid
    /// ```.rust
    /// let mut grid = das_grid::Grid::new((3, 2), (1., 1.), 1);
    /// assert_eq!(grid.cols(), 2);
    /// ```
    pub fn cols(&self) -> i32 {
        self.frame_size.1
    }

    /// Return the cell_size specified in grid creation
    ///
    /// ```.rust
    /// let grid = das_grid::Grid::new((4, 4), (10., 10.), 0);
    /// assert_eq!(grid.get_cell_size(), (10., 10.))
    /// ```
    pub fn get_cell_size(&self) -> (f32, f32) {
        self.cell_size
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

impl<T: Copy> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (rows, cols) = self.frame_size;
        write!(f, "Grid {{ rows: {}, cols: {}, cells: [...] }}", rows, cols)
    }
}

impl<T: Copy + Display> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cell_str = String::new();
        let (rows, cols) = self.frame_size;

        for (x, y, v) in self.enumerate_with_value() {
            if y as i32 % cols == 0 && x > 0 {
                cell_str += "\n";
            }

            cell_str.push_str(&format!("  {:10} (x: {} y: {})", v, x, y));
        }

        write!(
            f,
            "Grid {{ rows: {}, cols: {}, cells: [\n{}\n] }}",
            rows, cols, cell_str,
        )
    }
}

#[cfg(test)]
#[path = "./grid_test.rs"]
mod grid_test;
