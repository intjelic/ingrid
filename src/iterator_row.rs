// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::iter::Iterator;
use crate::coordinate::Coordinate;
use crate::row::Row;
use crate::grid_iterator::GridIterator;
use crate::coord;

/// An iterator over a row
///
/// This structure is an iterator over the elements of a row. It's constructed
/// from a row directly.
///
/// # Examples
///
/// ```
/// # use ingrid::Grid;
/// #
/// let grid = Grid::from_rows(vec![vec![1, 2],
///                                 vec![3, 4]]);
///
/// let mut iterator = grid.row(1).iterator();
/// assert_eq!(iterator.next(), Some(&3));
/// assert_eq!(iterator.next(), Some(&4));
/// assert_eq!(iterator.next(), None);
/// ```
///
pub struct IteratorRow<'a, T> {
    row: Row<'a, T>,
    index: usize
}

impl<'a, T> IteratorRow<'a, T> {
    pub fn new(row: Row<'a, T>) -> IteratorRow<'a, T> {
        IteratorRow { row, index: 0 }
    }
}

impl<'a, T: Clone> Iterator for IteratorRow<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.row.length() {
            None
        }
        else {
            let value = self.row.value(self.index);
            self.index += 1;
            Some(value)
        }
    }
}

impl<'a, T: Clone> GridIterator for IteratorRow<'a, T> {
    fn coordinate(&self) -> Coordinate {
        coord!(self.index, self.row.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn iterator_row() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                           vec![4, 5, 6]]);

        let mut iterator = IteratorRow::new(grid.row(0));

        assert_eq!(iterator.coordinate(), coord!(0, 0));
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.coordinate(), coord!(1, 0));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.coordinate(), coord!(2, 0));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.coordinate(), coord!(3, 0));
        assert_eq!(iterator.next(), None);

        let mut iterator = IteratorRow::new(grid.row(1));

        assert_eq!(iterator.coordinate(), coord!(0, 1));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.coordinate(), coord!(1, 1));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.coordinate(), coord!(2, 1));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.coordinate(), coord!(3, 1));
        assert_eq!(iterator.next(), None);
    }
}