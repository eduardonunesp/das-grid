#![allow(warnings, unused)]

use std::{
    borrow::BorrowMut,
    fmt::{self},
};

use crate::{Grid, GridErr, MoveDirection};

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

#[test]
fn test_create_grid() {
    let g = Grid::new(10, 10, 0);
    assert!(g.size() == 100);
}

#[test]
fn test_create_grid_with_str() {
    let g: Grid<&str> = Grid::new(10, 10, "a");
    assert!(g.get((0, 0)).unwrap() == &"a");
}

#[test]
fn test_create_grid_with_enum() {
    let mut g: Grid<Pawn> = Grid::new(10, 10, Pawn::None);
    assert!(g.get((0, 0)).unwrap() == &Pawn::None);

    g.set((5, 5), &Pawn::Player);
    assert!(g.mov_to((5, 5), MoveDirection::Right).is_ok());
}

#[test]
#[should_panic]
fn test_forbidden_grid_size() {
    let g = Grid::new(0, 0, 0);
    assert!(g.size() == 0);
}

#[test]
fn test_iterate_on_grid() {
    let g = Grid::new(10, 10, 1);
    assert!(g.into_iter().sum::<i32>() == 100);
}
#[test]
fn test_get_pos() {
    let g = Grid::new(2, 2, 1);
    assert!(g.get((0, 0)).unwrap_or(&0) == &1);
}

#[test]
fn test_get_mut_pos() {
    let mut g = Grid::new(2, 2, 1);
    let p = g.get_mut((0, 0)).unwrap();
    *p = 50;
    assert!(g.get((0, 0)).unwrap_or(&0) == &50);
}

#[test]
fn test_set_value() {
    let mut g = Grid::new(2, 2, 1);
    let p = g.get_mut((0, 0)).unwrap();
    *p = 50;
    g.set((0, 0), &2);
    assert!(g.get((0, 0)).unwrap() == &2);
}

#[test]
fn test_mov_cell() {
    let mut g = Grid::new(2, 2, 0);
    let mut count = 1;
    for c in &mut g {
        *c = count;
        count += 1;
    }
    g.mov((0, 0), (1, 1));
    assert!(g.get((1, 1)).unwrap() == &1);
}

#[test]
fn test_move_to() {
    let mut g = Grid::new(2, 2, 0);
    g.set((0, 0), &1);

    let ret = g.mov_to((0, 0), MoveDirection::Right);
    assert!(g.get((0, 1)).unwrap() == &1);
    assert!(ret.is_ok());

    let ret = g.mov_to((0, 1), MoveDirection::Right);
    assert!(ret.unwrap_err() == GridErr::OutOfGrid);

    let ret = g.mov_to((0, 1), MoveDirection::Left);
    assert!(g.get((0, 0)).unwrap() == &1);

    let ret = g.mov_to((0, 0), MoveDirection::Left);
    assert!(ret.unwrap_err() == GridErr::OutOfGrid);

    let ret = g.mov_to((0, 0), MoveDirection::Up);
    assert!(ret.unwrap_err() == GridErr::OutOfGrid);

    let ret = g.mov_to((0, 0), MoveDirection::Down);
    assert!(ret.is_ok());

    let ret = g.mov_to((1, 0), MoveDirection::Down);
    assert!(ret.unwrap_err() == GridErr::OutOfGrid);

    let ret = g.mov_to((1, 0), MoveDirection::Right);
    assert!(ret.is_ok());

    let ret = g.mov_to((1, 1), MoveDirection::Right);
    assert!(ret.unwrap_err() == GridErr::OutOfGrid);
}

#[test]
fn test_enumerate() {
    let mut grid: Grid<i32> = Grid::new(2, 2, 0);

    let mut result: Vec<(i32, i32)> = vec![];
    for xy in grid.enumerate() {
        result.push(xy)
    }
    assert!(result == [(0, 0), (1, 0), (0, 1), (1, 1)])
}

#[test]
fn test_iterators() {
    let grid: Grid<i32> = Grid::new(2, 2, 0);

    let mut result: Vec<i32> = vec![];
    for v in &grid {
        result.push(*v);
    }

    assert!(result == [0, 0, 0, 0]);
}

#[test]
fn test_set_with_rules() {
    let mut grid: Grid<i32> = Grid::new(2, 2, 0);
    assert!(grid.set((0, 1), &1).is_ok());

    let rule_not_1 = |_: (i32, i32), value: &i32| -> Result<(), GridErr> {
        if *value == 1 {
            return Err(GridErr::RuleFailed);
        }
        Ok(())
    };

    assert!(
        grid.set_with_rules((0, 1), &1, vec![rule_not_1])
            .err()
            .unwrap()
            == GridErr::RuleFailed
    );
}

#[test]
fn test_stamp_subgrid() {
    let mut grid: Grid<i32> = Grid::new(10, 10, 0);
    let sub_grid: Grid<i32> = Grid::new(2, 2, 1);
    assert!(grid.stamp_subgrid((5, 5), sub_grid).is_ok());
    assert!(grid.get((5, 5)).unwrap() == &1);
    assert!(grid.get((5, 6)).unwrap() == &1);
    assert!(grid.get((6, 5)).unwrap() == &1);
    assert!(grid.get((6, 6)).unwrap() == &1);
}

#[test]
fn test_stamp_subgrid_with_rules_1() {
    let mut grid: Grid<i32> = Grid::new(10, 10, 1);
    let sub_grid: Grid<i32> = Grid::new(2, 2, 1);

    let rule_not_1 = |_: (i32, i32), value: &i32| -> Result<(), GridErr> {
        if *value == 1 {
            return Err(GridErr::RuleFailed);
        }
        Ok(())
    };

    assert!(grid
        .stamp_subgrid_with_rules((5, 5), sub_grid, vec![rule_not_1])
        .is_err());
}

#[test]
fn test_get_row() {
    let mut g = Grid::new_from_vector(2, 2, vec![1, 2, 3, 4]);
    let row = g.get_row(1).unwrap();
    assert_eq!(row, vec![3, 4]);
    let col = g.get_col(1).unwrap();
    assert_eq!(col, vec![2, 4]);
}

#[test]
fn test_new_from_vec() {
    let mut g = Grid::new_from_vector(2, 2, vec![1, 2, 3, 4]);
    assert_eq!(g.size(), 4);
}

#[test]
fn test_get_subgrid() {
    let mut grid = Grid::new_from_vector(4, 4, (1..=16).collect());
    let sub_grid = grid.get_subgrid((2, 2), 2, 2).unwrap();
    assert_eq!(sub_grid.get_flatten_grid(), vec![11, 12, 15, 16]);
}

#[test]
fn test_mov_to_with_rules() {
    let mut g = Grid::new(2, 2, 0);
    g.set((0, 1), &1);

    let rule_not_1 = |_: (i32, i32), value: &i32| -> Result<(), GridErr> {
        if *value == 1 {
            return Err(GridErr::RuleFailed);
        }
        Ok(())
    };

    let ret = g.mov_to_with_rules((0, 0), MoveDirection::Right, vec![rule_not_1]);
    assert!(ret.is_err());
}

// #[test]
// fn test_generic() {
//     let mut g = Grid::new(3, 3, 0);
//     g.set((0, 1), &1);
//     g.debug();
//     println!("{:?}", g.mov_to((0, 1), MoveDirection::Right));
//     g.debug();
// }
