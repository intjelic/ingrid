// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use crate::coordinate::Coordinate;
use crate::grid::Grid;

/// A cell intermediary accessor
///
/// This structure is not implemented yet.
///
pub struct Cell<'a, T> {
    grid: &'a Grid<T>,
    coordinate: Coordinate
}

impl<'a, T> Cell<'a, T> {
    pub fn value() {}
    pub fn value_mut() {}
    pub fn set_value() {}
    pub fn swap_value() {}

    pub fn top_left() -> Option<&'a T> { None }
    pub fn top() -> Option<&'a T> { None }
    pub fn top_right() -> Option<&'a T> { None }

    pub fn left() -> Option<&'a T> { None }
    pub fn right() -> Option<&'a T> { None }

    pub fn bottom_left() -> Option<&'a T> { None }
    pub fn bottom() -> Option<&'a T> { None }
    pub fn bottom_right() -> Option<&'a T> { None }
}