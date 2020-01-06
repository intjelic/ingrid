// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::iter::Iterator;
use crate::coordinate::Coordinate;
use crate::column::Column;
use crate::grid_iterator::GridIterator;
use crate::coord;

/// An iterator over a column
///
/// This structure is an iterator over the elements of a column. It's
/// constructed from a column directly.
///
/// # Examples
///
/// ```
/// # use ingrid::Grid;
/// #
/// let grid = Grid::from_rows(vec![vec![1, 2],
///                                 vec![3, 4]]);
///
/// let mut iterator = grid.column(1).iterator();
/// assert_eq!(iterator.next(), Some(&2));
/// assert_eq!(iterator.next(), Some(&4));
/// assert_eq!(iterator.next(), None);
/// ```
///
pub struct IteratorColumn<'a, T> {
    column: Column<'a, T>,
    index: usize
}

impl<'a, T> IteratorColumn<'a, T> {
    pub fn new(column: Column<'a, T>) -> IteratorColumn<'a, T> {
        IteratorColumn { column, index: 0 }
    }
}

impl<'a, T: Clone> Iterator for IteratorColumn<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.column.length() {
            None
        }
        else {
            let value = self.column.value(self.index);
            self.index += 1;
            Some(value)
        }
    }
}

impl<'a, T: Clone> GridIterator for IteratorColumn<'a, T> {
    fn coordinate(&self) -> Coordinate {
        coord!(self.column.index, self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn iterator_column() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                           vec![3, 4],
                                           vec![5, 6]]);

        let mut iterator = IteratorColumn::new(grid.column(0));

        assert_eq!(iterator.coordinate(), coord!(0, 0));
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.coordinate(), coord!(0, 1));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.coordinate(), coord!(0, 2));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.coordinate(), coord!(0, 3));
        assert_eq!(iterator.next(), None);

        let mut iterator = IteratorColumn::new(grid.column(1));

        assert_eq!(iterator.coordinate(), coord!(1, 0));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.coordinate(), coord!(1, 1));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.coordinate(), coord!(1, 2));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.coordinate(), coord!(1, 3));
        assert_eq!(iterator.next(), None);
    }
}