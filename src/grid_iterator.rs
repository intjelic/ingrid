// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::iter::Iterator;
use crate::coordinate::Coordinate;
use crate::enumerate_coordinate::EnumerateCoordinate;

/// An interface to implement grid iterators
///
/// This trait allows to implement an iterator for a grid. A grid iterator has
/// the particularity of providing additional adaptors through its provided
/// methods (for now, only `enumerate_coordinate()`). It must also be able to
/// return the current coordinate.
///
/// Note that a grid iterator implements the standard iterator interface.
///
pub trait GridIterator : Iterator {
    fn coordinate(&self) -> Coordinate;

    fn enumerate_coordinate(self) -> EnumerateCoordinate<Self> where Self: Sized {
        EnumerateCoordinate::new(self)
    }
}