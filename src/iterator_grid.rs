// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::iter::Iterator;
use crate::coordinate::Coordinate;
use crate::grid::Grid;
use crate::grid_iterator::GridIterator;
use crate::coord;

/// An iterator over a grid
///
/// This structure is an iterator over the elements of a grid. It's constructed
/// from the grid directly.
///
/// # Examples
///
/// ```
/// # use ingrid::Grid;
/// #
/// let grid = Grid::from_rows(vec![vec![1, 2],
///                                 vec![3, 4]]);
///
/// let mut iterator = grid.iterator();
/// assert_eq!(iterator.next(), Some(&1));
/// assert_eq!(iterator.next(), Some(&2));
/// assert_eq!(iterator.next(), Some(&3));
/// assert_eq!(iterator.next(), Some(&4));
/// assert_eq!(iterator.next(), None);
/// ```
///
pub struct IteratorGrid<'a, T> {
    grid: &'a Grid<T>,
    coordinate: Coordinate
}

impl<'a, T> IteratorGrid<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> IteratorGrid<'a, T> {
        IteratorGrid { grid, coordinate: coord!(0, 0) }
    }
}

impl<'a, T: Clone> Iterator for IteratorGrid<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coordinate.y == self.grid.size().height {
            None
        }
        else {
            let value = self.grid.value(self.coordinate);

            self.coordinate.x += 1;
            if self.coordinate.x == self.grid.size().width {
                self.coordinate.x = 0;
                self.coordinate.y += 1;
            }

            Some(value)
        }
    }
}

impl<'a, T: Clone> GridIterator for IteratorGrid<'a, T> {
    fn coordinate(&self) -> Coordinate {
        self.coordinate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn iterator_from_grid() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        let mut iterator = IteratorGrid::new(&grid);

        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), Some(&7));
        assert_eq!(iterator.next(), Some(&8));
        assert_eq!(iterator.next(), Some(&9));
        assert_eq!(iterator.next(), None);
    }
}