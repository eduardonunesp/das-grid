#![allow(warnings, unused)]
#[macro_use]
use pretty_assertions::{assert_eq, assert_ne};
use parse_display_derive::Display;

use std::{borrow::BorrowMut, fmt};

use crate::MoveDirection;
use crate::Result;
use crate::{DasGridError, Grid};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display)]
enum Pawn {
    #[display("None")]
    None,
    #[display("Player")]
    Player,
}

#[test]
fn test_create_grid() {
    let g = Grid::new((10, 10), (1., 1.), 0);
    assert_eq!(g.size(), 100);
}

#[test]
fn test_create_grid_with_str() {
    let g: Grid<&str> = Grid::new((10, 10), (1., 1.), "a");
    assert_eq!(g.get((0, 0)).unwrap(), &"a");
}

#[test]
fn test_create_grid_with_enum() {
    let mut g: Grid<Pawn> = Grid::new((10, 10), (1., 1.), Pawn::None);
    assert_eq!(g.get((0, 0)).unwrap(), &Pawn::None);
    g.set((5, 5), &Pawn::Player);
    assert_eq!(g.mov_to((5, 5), MoveDirection::Right).is_ok(), true);
}

#[test]
#[should_panic]
fn test_forbidden_grid_size() {
    let g = Grid::new((0, 0), (1., 1.), 0);
    assert_eq!(g.size(), 0);
}

#[test]
fn test_iterate_on_grid() {
    let g = Grid::new((10, 10), (1., 1.), 1);
    assert_eq!(g.into_iter().sum::<i32>(), 100);
}
#[test]
fn test_get_pos() {
    let g = Grid::new((2, 2), (1., 1.), 1);
    assert_eq!(g.get((0, 0)).unwrap_or(&0), &1);
}

#[test]
fn test_get_mut_pos() {
    let mut g = Grid::new((2, 2), (1., 1.), 1);
    let p = g.get_mut((0, 0)).unwrap();
    *p = 50;
    assert_eq!(g.get((0, 0)).unwrap_or(&0), &50);
}

#[test]
fn test_set_value() {
    let mut g = Grid::new((2, 2), (1., 1.), 1);
    let p = g.get_mut((0, 0)).unwrap();
    *p = 50;
    g.set((0, 0), &2);
    assert_eq!(g.get((0, 0)).unwrap(), &2);
}

#[test]
fn test_mov_cell() {
    let mut g = Grid::new((2, 2), (1., 1.), 0);
    let mut count = 1;
    for c in &mut g {
        *c = count;
        count += 1;
    }
    g.mov((0, 0), (1, 1));
    assert_eq!(g.get((1, 1)).unwrap(), &1);
}

#[test]
fn test_move_to() {
    let mut g = Grid::new((2, 2), (1., 1.), 0);
    g.set((0, 0), &1);

    let ret = g.mov_to((0, 0), MoveDirection::Down);
    assert_eq!(g.get((0, 1)).unwrap(), &1);
    assert_eq!(ret.is_ok(), true);

    let ret = g.mov_to((0, 1), MoveDirection::Down);
    assert_eq!(ret.unwrap_err(), DasGridError::OutOfGrid);

    let ret = g.mov_to((0, 1), MoveDirection::Up);
    assert_eq!(g.get((0, 0)).unwrap(), &1);

    let ret = g.mov_to((0, 0), MoveDirection::Up);
    assert_eq!(ret.unwrap_err(), DasGridError::OutOfGrid);

    let ret = g.mov_to((0, 0), MoveDirection::Up);
    assert_eq!(ret.unwrap_err(), DasGridError::OutOfGrid);

    let ret = g.mov_to((0, 0), MoveDirection::Down);
    assert_eq!(ret.is_ok(), true);

    let ret = g.mov_to((1, 0), MoveDirection::Right);
    assert_eq!(ret.unwrap_err(), DasGridError::OutOfGrid);

    let ret = g.mov_to((1, 0), MoveDirection::Down);
    assert_eq!(ret.is_ok(), true);

    let ret = g.mov_to((1, 1), MoveDirection::Right);
    assert_eq!(ret.unwrap_err(), DasGridError::OutOfGrid);
}

#[test]
fn test_enumerate() {
    let mut grid: Grid<i32> = Grid::new((2, 2), (1., 1.), 1);

    let mut result: Vec<(i32, i32, &i32)> = vec![];
    for xy in grid.enumerate_with_value() {
        result.push(xy);
    }
    assert_eq!(result, [(0, 0, &1), (0, 1, &1), (1, 0, &1), (1, 1, &1)])
}

#[test]
fn test_iterators() {
    let grid: Grid<i32> = Grid::new((2, 2), (1., 1.), 0);

    let mut result: Vec<i32> = vec![];
    for v in &grid {
        result.push(*v);
    }

    assert_eq!(result, [0, 0, 0, 0]);
}

#[test]
fn test_set_with_rules() {
    let mut grid: Grid<i32> = Grid::new((2, 2), (1., 1.), 0);
    assert_eq!(grid.set((0, 1), &1).is_ok(), true);

    let rule_not_1 = |_: (i32, i32), value: &i32| -> Result {
        if *value == 1 {
            return Err(DasGridError::RuleFailed);
        }
        Ok(())
    };

    assert_eq!(
        grid.set_with_rules((0, 1), &1, vec![rule_not_1])
            .err()
            .unwrap(),
        DasGridError::RuleFailed
    );
}

#[test]
fn test_stamp_subgrid() {
    let mut grid: Grid<i32> = Grid::new((10, 10), (1., 1.), 0);
    let sub_grid: Grid<i32> = Grid::new((2, 2), (1., 1.), 1);
    assert_eq!(grid.stamp_subgrid((5, 5), &sub_grid).is_ok(), true);
    assert_eq!(grid.get((5, 5)).unwrap(), &1);
    assert_eq!(grid.get((5, 6)).unwrap(), &1);
    assert_eq!(grid.get((6, 5)).unwrap(), &1);
    assert_eq!(grid.get((6, 6)).unwrap(), &1);
}

#[test]
fn test_stamp_subgrid_with_rules_1() {
    let mut grid: Grid<i32> = Grid::new((10, 10), (1., 1.), 1);
    let sub_grid: Grid<i32> = Grid::new((2, 2), (1., 1.), 1);

    let rule_not_1 = |_: (i32, i32), value: &i32| -> Result {
        if *value == 1 {
            return Err(DasGridError::RuleFailed);
        }
        Ok(())
    };

    assert_eq!(
        grid.stamp_subgrid_with_rules((5, 5), sub_grid, vec![rule_not_1])
            .is_err(),
        true
    );
}

#[test]
fn test_get_row() {
    let mut g = Grid::new_from_vector((2, 2), (1., 1.), vec![1, 2, 3, 4]);
    let row = g.get_row(1).unwrap();
    assert_eq!(row, vec![3, 4]);
    let col = g.get_col(1).unwrap();
    assert_eq!(col, vec![2, 4]);
}

#[test]
fn test_new_from_vec() {
    let mut g = Grid::new_from_vector((2, 2), (1., 1.), vec![1, 2, 3, 4]);
    assert_eq!(g.size(), 4);
}

#[test]
fn test_get_subgrid() {
    let mut grid = Grid::new_from_vector((4, 4), (1., 1.), (1..=16).collect());
    let sub_grid = grid.get_subgrid((2, 2), 2, 2).unwrap();
    assert_eq!(sub_grid.get_flatten_grid(), vec![11, 12, 15, 16]);
}

#[test]
fn test_mov_to_with_rules() {
    let mut g = Grid::new((2, 2), (1., 1.), 0);
    g.set((0, 1), &1);

    let rule_not_1 = |_: (i32, i32), value: &i32| -> Result {
        if *value == 1 {
            return Err(DasGridError::RuleFailed);
        }
        Ok(())
    };

    let ret = g.mov_to_with_rules((0, 0), MoveDirection::Down, vec![rule_not_1]);
    assert_eq!(ret.is_err(), true);
}

#[test]
fn test_fill_subgrid() {
    let mut grid = Grid::new_from_vector((4, 4), (1., 1.), (1..=16).collect());
    grid.fill_subgrid((1, 1), (2, 2), &0);
    assert_eq!(grid.get((1, 1)).unwrap(), &0);
    assert_eq!(grid.get((1, 2)).unwrap(), &0);
    assert_eq!(grid.get((2, 1)).unwrap(), &0);
    assert_eq!(grid.get((2, 2)).unwrap(), &0);
}

#[test]
fn test_rotate_cw() {
    let mut grid = Grid::new_from_vector((3, 3), (1., 1.), (1..=9).collect());
    let initial_cells = grid.cells.clone();

    // Rotate matrix for times
    for _ in (0..4) {
        grid.rotate_cw();
    }

    let ending_position = grid.cells.clone();
    assert_eq!(initial_cells, ending_position);

    let mut grid = Grid::new_from_vector((4, 4), (1., 1.), (1..=16).collect());
    let initial_cells = grid.cells.clone();

    // Rotate matrix for times
    for _ in (0..4) {
        grid.rotate_cw();
    }

    let ending_position = grid.cells.clone();
    assert_eq!(initial_cells, ending_position);
}

#[test]
fn test_rotate_ccw() {
    let mut grid = Grid::new_from_vector((3, 3), (1., 1.), (1..=9).collect());
    let initial_cells = grid.cells.clone();

    // Rotate matrix for times
    for _ in (0..4) {
        grid.rotate_ccw();
        grid.debug();
    }

    let ending_position = grid.cells.clone();
    assert_eq!(initial_cells, ending_position);
}
