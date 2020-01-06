// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::ops::Index;
use crate::coordinate::Coordinate;
use crate::grid::Grid;
use crate::iterator_row::IteratorRow;
use crate::coord;

/// A view onto a row of a grid
///
/// This structure is an **immutable** view into a row of a grid and its
/// **lifetime is bound** to the lifetime of the grid. It's a **lightweight**
/// construct that allows to operate on individual rows effectively; see it as
/// an equivalent to the **slice primitive** for grids.
///
/// Instead of accessing **elements** with coordinates, just an **index** is
/// needed. Rows use the **left to right** direction, therefore, index zero
/// corresponds to the element at the very left, also denoted the 'first'
/// element of the row. Note that rows are indexable.
///
/// With a row, you can easily retrieve the left and right elements of the row,
/// with the `left()` and `right()` methods, but also retrieve the row above or
/// below, with the `top()` and `bottom()` methods. You can also conveniently
/// iterate over its elements with the `iterator()` method which returns an
/// efficient iterator.
///
/// Because this view is **immutable**, it's limited in terms of what it can do;
/// check out the **mutable** counter-part for more operations over the rows.
///
/// # Examples
///
/// Iterating over the elements of a row.
///
/// ```
/// # use ingrid::{Grid, GridIterator};
/// #
/// let grid = Grid::from_rows(vec![vec![1, 2, 3],
///                                 vec![4, 5, 6]]);
///
/// let row = grid.row(0);
/// for (coordinate, value) in row.iterator().enumerate_coordinate() {
///     println!("Element at {:?} has value {}.", coordinate, *value);
/// }
/// ```
///
/// Indexing the row.
///
/// ```
/// # use ingrid::Grid;
/// #
/// let grid = Grid::from_rows(vec![vec![1, 2, 3],
///                                 vec![4, 5, 6]]);
///
/// println!("First element of first row is {}", grid.row(0)[0]);
/// println!("Last element of last row is {}", grid.row(1)[2]);
/// ```
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Row<'a, T> {
    /// A reference to its grid.
    pub grid: &'a Grid<T>,

    /// The index of the row.
    pub index: usize
}

impl<'a, T: Clone> Row<'a, T> {

    /// Returns the length of the row.
    ///
    /// This method returns the length of the row which is the number of
    /// elements. It's equivalent to the width of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let grid = Grid::with_size(size!(3, 2), 42);
    ///
    /// assert_eq!(grid.row(0).length(), 3);
    /// assert_eq!(grid.row(1).length(), 3);
    /// assert_eq!(grid.size().width, 3);
    /// ```
    ///
    pub fn length(&self) -> usize {
        self.grid.size().width
    }

    /// Returns a reference to an element of the row.
    ///
    /// This method returns a reference to an element of the row from its index.
    ///
    /// Note that index zero corresponds to the element at the very left (the
    /// first element of the row). If you're looking to get the first or the
    /// last elements of the row, check out the `left()` and `right()` methods.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the element
    ///
    /// # Panics
    ///
    /// It panics if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                 vec![4, 5, 6]]);
    ///
    /// let row = grid.row(1);
    /// assert_eq!(row.value(0), &4);
    /// assert_eq!(row.value(1), &5);
    /// assert_eq!(row.value(2), &6);
    /// row.value(3); // It panics here !
    /// ```
    ///
    pub fn value(&self, index: usize) -> &'a T {
        self.grid.value(coord!(index, self.index))
    }

    /// Return the elements of the row.
    ///
    /// This method returns the elements of the row as a vector of reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    ///
    /// assert_eq!(grid.row(0).values(), vec![&1, &2]);
    /// assert_eq!(grid.row(1).values(), vec![&3, &4]);
    /// ```
    ///
    pub fn values(&self) -> Vec<&T> {
        self.iterator().collect()
    }

    /// Returns a reference to the first element of the row.
    ///
    /// This method returns a reference to the first element of the row. It's
    /// equivalent to retrieving the element with index `0`.
    ///
    /// Note that there is always a first element or the grid would have no
    /// size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                 vec![4, 5, 6]]);
    ///
    /// // The first element of the second row is 4.
    /// let row = grid.row(1);
    /// assert_eq!(row.left(), &4);
    /// ```
    ///
    pub fn left(&self) -> &T {
        self.grid.value(coord!(0, self.index))
    }

    /// Returns a reference to the last element of the row.
    ///
    /// This method returns a reference to the last element of the row. It's
    /// equivalent to retrieving the element with index `length() -1`.
    ///
    /// Note that there is always a last element or the grid would have no
    /// size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                 vec![4, 5, 6]]);
    ///
    /// // The last element of the second row is 6.
    /// let row = grid.row(1);
    /// assert_eq!(row.right(), &6);
    /// ```
    ///
    pub fn right(&self) -> &T {
        self.grid.value(coord!(self.grid.size().width-1, self.index))
    }

    /// Returns an iterator over the row.
    ///
    /// This method returns an iterator over the row.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![ 1,  2,  3],
    ///                                 vec![42, 42, 42]]);
    ///
    /// // Check if all elements of the row have value 42.
    /// assert_eq!(grid.row(0).iterator().all(|item| *item == 42), false);
    /// assert_eq!(grid.row(1).iterator().all(|item| *item == 42), true);
    /// ```
    ///
    pub fn iterator(&self) -> IteratorRow<'a, T> {
        IteratorRow::new(self.clone())
    }

    /// Returns the row above.
    ///
    /// This method returns the row above this row, or `None` if this is already
    /// the row at the very top of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                    vec![3, 4]]);
    ///
    /// let second_row = grid.row(1);
    /// let first_row = second_row.top().unwrap();
    /// assert!(first_row.top().is_none()); // There is no row above.
    /// ```
    ///
    pub fn top(&self) -> Option<Row<'a, T>> {
        match self.index {
            0 => None,
            index => Some(self.grid.row(index - 1))
        }
    }

    /// Returns the row below.
    ///
    /// This method returns the row below this row, or `None` if this is already
    /// the row at the very bottom of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    ///
    /// let first_row = grid.row(0);
    /// let second_row = first_row.bottom().unwrap();
    /// assert!(second_row.bottom().is_none()); // There is no row below.
    /// ```
    ///
    pub fn bottom(&self) -> Option<Row<'a, T>> {
        // rework this to use match syntax
        if self.index == self.length() -1 {
            None
        }
        else {
            Some(self.grid.row(self.index + 1))
        }
    }
}

impl<'a, T: Clone> Index<usize> for Row<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.value(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_length() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        assert_eq!(grid.row(0).length(), 3);
        assert_eq!(grid.row(1).length(), 3);
        assert_eq!(grid.row(2).length(), 3);
    }


    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_value() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        let row = grid.row(0);
        assert_eq!(row.value(0), &1);
        assert_eq!(row.value(1), &2);

        let row = grid.row(1);
        assert_eq!(row.value(0), &3);
        assert_eq!(row.value(1), &4);

        row.value(2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_index() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        let row = grid.row(0);
        assert_eq!(row[0], 1);
        assert_eq!(row[1], 2);

        let row = grid.row(1);
        assert_eq!(row[0], 3);
        assert_eq!(row[1], 4);

        row[2];
    }

    #[test]
    fn row_left() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        assert_eq!(grid.row(0).left(), &1);
        assert_eq!(grid.row(1).left(), &4);
        assert_eq!(grid.row(2).left(), &7);
    }

    #[test]
    fn row_right() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        assert_eq!(grid.row(0).right(), &3);
        assert_eq!(grid.row(1).right(), &6);
        assert_eq!(grid.row(2).right(), &9);
    }

    #[test]
    fn row_iterator() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6]]);

        let mut iterator = grid.row(0).iterator();

        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), None);

        let mut iterator = grid.row(1).iterator();

        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn row_top() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        let last_row = grid.row(2);
        assert_eq!(last_row.values(), vec!(&7, &8, &9));

        let middle_row = last_row.top().unwrap();
        assert_eq!(middle_row.values(), vec!(&4, &5, &6));

        let first_row = middle_row.top().unwrap();
        assert_eq!(first_row.values(), vec!(&1, &2, &3));

        assert!(first_row.top().is_none());
    }

    #[test]
    fn row_bottom() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        let first_row = grid.row(0);
        assert_eq!(first_row.values(), vec!(&1, &2, &3));

        let middle_row = first_row.bottom().unwrap();
        assert_eq!(middle_row.values(), vec!(&4, &5, &6));

        let last_row = middle_row.bottom().unwrap();
        assert_eq!(last_row.values(), vec!(&7, &8, &9));

        assert!(last_row.bottom().is_none());
    }
}