#![allow(warnings, unused)]

use std::{
    borrow::BorrowMut,
    fmt::{self},
};

use crate::{Grid, MoveDirection, OutOfGridErr};

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
    assert!(g.get((1, 0)).unwrap() == &1);
    assert!(ret.is_ok());

    let ret = g.mov_to((1, 0), MoveDirection::Right);
    assert!(ret.unwrap_err() == OutOfGridErr);

    let ret = g.mov_to((1, 0), MoveDirection::Left);
    assert!(g.get((0, 0)).unwrap() == &1);

    let ret = g.mov_to((0, 0), MoveDirection::Left);
    assert!(ret.unwrap_err() == OutOfGridErr);

    let ret = g.mov_to((0, 0), MoveDirection::Up);
    assert!(ret.unwrap_err() == OutOfGridErr);

    let ret = g.mov_to((0, 0), MoveDirection::Down);
    assert!(ret.is_ok());

    let ret = g.mov_to((0, 1), MoveDirection::Down);
    assert!(ret.unwrap_err() == OutOfGridErr);

    let ret = g.mov_to((0, 1), MoveDirection::Right);
    assert!(ret.is_ok());

    let ret = g.mov_to((1, 1), MoveDirection::Right);
    assert!(ret.unwrap_err() == OutOfGridErr);
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
