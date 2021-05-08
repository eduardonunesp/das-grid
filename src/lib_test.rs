#![allow(warnings, unused)]

use std::fmt::{self};

use crate::{DasGrid, OutOfGridErr};

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
    let g = DasGrid::new(10, 10, 0);
    assert!(g.size() == 100);
}

#[test]
fn test_create_grid_with_str() {
    let g: DasGrid<&str> = DasGrid::new(10, 10, "a");
    assert!(g.get((0, 0)).unwrap() == &"a");
}

#[test]
fn test_create_grid_with_enum() {
    let mut g: DasGrid<Pawn> = DasGrid::new(10, 10, Pawn::None);
    assert!(g.get((0, 0)).unwrap() == &Pawn::None);

    g.set((5, 5), &Pawn::Player);
    assert!(g.mov_to((5, 5), crate::MoveDirection::Right).is_ok());
}

#[test]
#[should_panic]
fn test_forbidden_grid_size() {
    let g = DasGrid::new(0, 0, 0);
    assert!(g.size() == 0);
}

#[test]
fn test_iterate_on_grid() {
    let g = DasGrid::new(10, 10, 1);
    assert!(g.into_iter().sum::<i32>() == 100);
}
#[test]
fn test_get_pos() {
    let g = DasGrid::new(2, 2, 1);
    assert!(g.get((0, 0)).unwrap_or(&0) == &1);
}

#[test]
fn test_get_mut_pos() {
    let mut g = DasGrid::new(2, 2, 1);
    let p = g.get_mut((0, 0)).unwrap();
    *p = 50;
    assert!(g.get((0, 0)).unwrap_or(&0) == &50);
}

#[test]
fn test_set_value() {
    let mut g = DasGrid::new(2, 2, 1);
    let p = g.get_mut((0, 0)).unwrap();
    *p = 50;
    g.set((0, 0), &2);
    assert!(g.get((0, 0)).unwrap() == &2);
}

#[test]
fn test_mov_cell() {
    let mut g = DasGrid::new(2, 2, 0);
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
    let mut g = DasGrid::new(2, 2, 0);
    g.set((0, 0), &1);

    let ret = g.mov_to((0, 0), crate::MoveDirection::Right);
    assert!(g.get((1, 0)).unwrap() == &1);
    assert!(ret.is_ok());

    let ret = g.mov_to((1, 0), crate::MoveDirection::Right);
    assert!(ret.unwrap_err() == OutOfGridErr);

    let ret = g.mov_to((1, 0), crate::MoveDirection::Left);
    assert!(g.get((0, 0)).unwrap() == &1);

    let ret = g.mov_to((0, 0), crate::MoveDirection::Left);
    assert!(ret.unwrap_err() == OutOfGridErr);

    let ret = g.mov_to((0, 0), crate::MoveDirection::Up);
    assert!(ret.unwrap_err() == OutOfGridErr);

    let ret = g.mov_to((0, 0), crate::MoveDirection::Down);
    assert!(ret.is_ok());

    let ret = g.mov_to((0, 1), crate::MoveDirection::Down);
    assert!(ret.unwrap_err() == OutOfGridErr);

    let ret = g.mov_to((0, 1), crate::MoveDirection::Right);
    assert!(ret.is_ok());

    let ret = g.mov_to((1, 1), crate::MoveDirection::Right);
    assert!(ret.unwrap_err() == OutOfGridErr);
}
