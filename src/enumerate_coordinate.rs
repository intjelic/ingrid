// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use crate::coordinate::Coordinate;
use crate::grid_iterator::GridIterator;

/// An iterator that yields the current coordinate
///
/// This structure is an iterator that yields the current coordinate and the
/// element of the grid during iteration. It's created by the
/// `enumerate_coordinate()` method on `GridIterator`.
///
/// # Examples
///
/// Iterating over a grid with the element coordinates.
///
/// ```
/// # use ingrid::{Coordinate, Grid, GridIterator, coord};
/// #
/// let grid = Grid::from_rows(vec![vec![1, 2],
///                                 vec![3, 4]]);
///
/// let mut iterator = grid.iterator().enumerate_coordinate();
/// assert_eq!(iterator.next(), Some((coord!(0, 0), &1)));
/// assert_eq!(iterator.next(), Some((coord!(1, 0), &2)));
/// assert_eq!(iterator.next(), Some((coord!(0, 1), &3)));
/// assert_eq!(iterator.next(), Some((coord!(1, 1), &4)));
/// assert_eq!(iterator.next(), None);
/// ```
///
/// Iterating over rows and columns with their element coordinates.
///
/// ```
/// # use ingrid::{Coordinate, Grid, GridIterator, coord};
/// #
/// let grid = Grid::from_rows(vec![vec![1, 2],
///                                 vec![3, 4]]);
///
/// let mut iterator = grid.row(0).iterator().enumerate_coordinate();
/// assert_eq!(iterator.next(), Some((coord!(0, 0), &1)));
/// assert_eq!(iterator.next(), Some((coord!(1, 0), &2)));
/// assert_eq!(iterator.next(), None);
///
/// let mut iterator = grid.column(1).iterator().enumerate_coordinate();
/// assert_eq!(iterator.next(), Some((coord!(1, 0), &2)));
/// assert_eq!(iterator.next(), Some((coord!(1, 1), &4)));
/// assert_eq!(iterator.next(), None);
/// ```
///
pub struct EnumerateCoordinate<I> {
    iterator: I
}

impl<I: GridIterator> EnumerateCoordinate<I> {
    pub fn new(iterator: I) -> EnumerateCoordinate<I> {
        EnumerateCoordinate {
            iterator
        }
    }
}

impl<I: GridIterator> Iterator for EnumerateCoordinate<I> {
    type Item = (Coordinate, <I as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let coordinate = self.iterator.coordinate();
        let value = self.iterator.next()?;

        Some((coordinate, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;
    use crate::iterator_grid::IteratorGrid;
    use crate::iterator_row::IteratorRow;
    use crate::iterator_column::IteratorColumn;

    #[test]
    fn enumerate_grid() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        let iterator = IteratorGrid::new(&grid);
        let mut enumerator = EnumerateCoordinate::new(iterator);

        assert_eq!(enumerator.next(), Some((coord!(0, 0), &1)));
        assert_eq!(enumerator.next(), Some((coord!(1, 0), &2)));
        assert_eq!(enumerator.next(), Some((coord!(2, 0), &3)));
        assert_eq!(enumerator.next(), Some((coord!(0, 1), &4)));
        assert_eq!(enumerator.next(), Some((coord!(1, 1), &5)));
        assert_eq!(enumerator.next(), Some((coord!(2, 1), &6)));
        assert_eq!(enumerator.next(), Some((coord!(0, 2), &7)));
        assert_eq!(enumerator.next(), Some((coord!(1, 2), &8)));
        assert_eq!(enumerator.next(), Some((coord!(2, 2), &9)));
        assert_eq!(enumerator.next(), None);
    }

    #[test]
    fn enumerate_row() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6]]);

        let iterator = IteratorRow::new(grid.row(0));
        let mut enumerator = EnumerateCoordinate::new(iterator);

        assert_eq!(enumerator.next(), Some((coord!(0, 0), &1)));
        assert_eq!(enumerator.next(), Some((coord!(1, 0), &2)));
        assert_eq!(enumerator.next(), Some((coord!(2, 0), &3)));
        assert_eq!(enumerator.next(), None);

        let iterator = IteratorRow::new(grid.row(1));
        let mut enumerator = EnumerateCoordinate::new(iterator);

        assert_eq!(enumerator.next(), Some((coord!(0, 1), &4)));
        assert_eq!(enumerator.next(), Some((coord!(1, 1), &5)));
        assert_eq!(enumerator.next(), Some((coord!(2, 1), &6)));
        assert_eq!(enumerator.next(), None);
    }

    #[test]
    fn enumerate_column() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4],
                                        vec![5, 6]]);

        let iterator = IteratorColumn::new(grid.column(0));
        let mut enumerator = EnumerateCoordinate::new(iterator);

        assert_eq!(enumerator.next(), Some((coord!(0, 0), &1)));
        assert_eq!(enumerator.next(), Some((coord!(0, 1), &3)));
        assert_eq!(enumerator.next(), Some((coord!(0, 2), &5)));
        assert_eq!(enumerator.next(), None);

        let iterator = IteratorColumn::new(grid.column(1));
        let mut enumerator = EnumerateCoordinate::new(iterator);

        assert_eq!(enumerator.next(), Some((coord!(1, 0), &2)));
        assert_eq!(enumerator.next(), Some((coord!(1, 1), &4)));
        assert_eq!(enumerator.next(), Some((coord!(1, 2), &6)));
        assert_eq!(enumerator.next(), None);
    }
}