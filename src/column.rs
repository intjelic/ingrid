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
use crate::iterator_column::IteratorColumn;
use crate::coord;

/// A view onto a column of a grid
///
/// This structure is an **immutable** view into a column of a grid and its
/// **lifetime is bound** to the lifetime of the grid. It's a **lightweight**
/// construct that allows to operate on individual columns effectively; see it
/// as an equivalent to the **slice primitive** for grids.
///
/// Instead of accessing **elements** with coordinates, just an **index** is
/// needed. Columns use the **top to bottom** direction, therefore, index zero
/// corresponds to the element at the very top, also denoted the 'first'
/// element of the column. Note that columns are indexable.
///
/// With a column, you can easily retrieve the top and bottom elements of the
/// column, with the `top()` and `bottom()` methods, but also retrieve the
/// column on the left or on the right, with the `left()` and `right()` methods.
/// You can also conveniently iterate over its elements with the `iterator()`
/// method which returns an efficient iterator.
///
/// Because this view is **immutable**, it's limited in terms of what it can do;
/// check out the **mutable** counter-part for more operations over the columns.
///
/// # Examples
///
/// Iterating over the elements of a column.
///
/// ```
/// # use ingrid::{Grid, GridIterator};
/// #
/// let grid = Grid::from_rows(vec![vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
///
/// let column = grid.column(0);
/// for (coordinate, value) in column.iterator().enumerate_coordinate() {
///     println!("Element at {:?} has value {}.", coordinate, *value);
/// }
/// ```
///
/// Indexing the column.
///
/// ```
/// # use ingrid::Grid;
/// #
/// let grid = Grid::from_rows(vec![vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
///
/// println!("First element of first column is {}", grid.column(0)[0]);
/// println!("Last element of last column is {}", grid.column(1)[2]);
/// ```
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Column<'a, T> {
    /// A reference to its grid.
    pub grid: &'a Grid<T>,

    /// The index of the column.
    pub index: usize
}

impl<'a, T: Clone> Column<'a, T> {

    /// Returns the length of the column.
    ///
    /// This method returns the length of the column which is the number of
    /// elements. It's equivalent to the height of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let grid = Grid::with_size(size!(2, 3), 42);
    ///
    /// assert_eq!(grid.column(0).length(), 3);
    /// assert_eq!(grid.column(1).length(), 3);
    /// assert_eq!(grid.size().height, 3);
    /// ```
    ///
    pub fn length(&self) -> usize {
        self.grid.size().height
    }

    /// Returns a reference to an element of the column.
    ///
    /// This method returns a reference to an element of the column from its
    /// index.
    ///
    /// Note that index zero corresponds to the element at the very top (the
    /// first element of the column). If you're looking to get the first or the
    /// last elements of the column, check out the `top()` and `bottom()`
    /// methods.
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
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4],
    ///                                 vec![5, 6]]);
    ///
    /// let column = grid.column(1);
    /// assert_eq!(column.value(0), &2);
    /// assert_eq!(column.value(1), &4);
    /// assert_eq!(column.value(2), &6);
    /// column.value(3); // It panics here !
    /// ```
    ///
    pub fn value(&self, index: usize) -> &'a T {
        self.grid.value(coord!(self.index, index))
    }

    /// Return the elements of the column.
    ///
    /// This method returns the elements of the column as a vector of reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    ///
    /// assert_eq!(grid.column(0).values(), vec![&1, &3]);
    /// assert_eq!(grid.column(1).values(), vec![&2, &4]);
    /// ```
    ///
    pub fn values(&self) -> Vec<&T> {
        self.iterator().collect()
    }

    /// Returns a reference to the first element of the column.
    ///
    /// This method returns a reference to the first element of the column. It's
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
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4],
    ///                                 vec![5, 6]]);
    ///
    /// // The first element of the second column is 2.
    /// let column = grid.column(1);
    /// assert_eq!(column.top(), &2);
    /// ```
    ///
    pub fn top(&self) -> &T {
        self.grid.value(coord!(self.index, 0))
    }

    /// Returns a reference to the last element of the column.
    ///
    /// This method returns a reference to the last element of the column. It's
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
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4],
    ///                                 vec![5, 6]]);
    ///
    /// // The last element of the second column is 6.
    /// let column = grid.column(1);
    /// assert_eq!(column.bottom(), &6);
    /// ```
    ///
    pub fn bottom(&self) -> &T {
        self.grid.value(coord!(self.index, self.grid.size().height-1))
    }

    /// Returns an iterator over the column.
    ///
    /// This method returns an iterator over the column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 42],
    ///                                 vec![3, 42],
    ///                                 vec![5, 42]]);
    ///
    /// // Check if all elements of the column have value 42.
    /// assert_eq!(grid.column(0).iterator().all(|item| *item == 42), false);
    /// assert_eq!(grid.column(1).iterator().all(|item| *item == 42), true);
    /// ```
    ///
    pub fn iterator(&self) -> IteratorColumn<'a, T> {
        IteratorColumn::new(self.clone())
    }

    /// Returns the column on the left.
    ///
    /// This method returns the column on the left of this column, or `None` if
    /// this is already the column at the very left of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4],
    ///                                 vec![5, 6]]);
    ///
    /// let second_column = grid.column(1);
    /// let first_column = second_column.left().unwrap();
    /// assert!(first_column.left().is_none()); // There is no column on the left.
    /// ```
    ///
    pub fn left(&self) -> Option<Column<'a, T>> {
        if self.index == 0 {
            None
        }
        else {
            // rework this
            let left_column_index: usize = (self.index - 1) as usize;
            Some(self.grid.column(left_column_index)) // remove integer conversation
        }
    }

    /// Returns the column on the right.
    ///
    /// This method returns the column on the right of this column, or `None` if
    /// this is already the column at the very right of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4],
    ///                                 vec![5, 6]]);
    ///
    /// let first_column = grid.column(0);
    /// let second_column = first_column.right().unwrap();
    /// assert!(second_column.right().is_none()); // There is no column on the right.
    /// ```
    ///
    pub fn right(&self) -> Option<Column<'a, T>> {
        if self.index == self.grid.size().width - 1 {
            None
        }
        else {
            // rework this
            let rigth_column_index: usize = (self.index + 1) as usize;
            Some(self.grid.column(rigth_column_index)) // remove integer conversation
        }
    }
}

impl<'a, T: Clone> Index<usize> for Column<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.value(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_length() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        assert_eq!(grid.column(0).length(), 3);
        assert_eq!(grid.column(1).length(), 3);
        assert_eq!(grid.column(2).length(), 3);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn column_value() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        let column = grid.column(0);
        assert_eq!(column.value(0), &1);
        assert_eq!(column.value(1), &3);

        let column = grid.column(1);
        assert_eq!(column.value(0), &2);
        assert_eq!(column.value(1), &4);

        column.value(2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn column_index() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        let column = grid.column(0);
        assert_eq!(column[0], 1);
        assert_eq!(column[1], 3);

        let column = grid.column(1);
        assert_eq!(column[0], 2);
        assert_eq!(column[1], 4);

        column[2];
    }

    #[test]
    fn column_top() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        assert_eq!(grid.column(0).top(), &1);
        assert_eq!(grid.column(1).top(), &2);
        assert_eq!(grid.column(2).top(), &3);
    }

    #[test]
    fn column_bottom() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        assert_eq!(grid.column(0).bottom(), &7);
        assert_eq!(grid.column(1).bottom(), &8);
        assert_eq!(grid.column(2).bottom(), &9);
    }

    #[test]
    fn column_iterator() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4],
                                        vec![5, 6]]);

        let mut iterator = grid.column(0).iterator();

        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), None);

        let mut iterator = grid.column(1).iterator();

        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn column_left() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        let last_column = grid.column(2);
        assert_eq!(last_column.values(), vec!(&3, &6, &9));

        let middle_column = last_column.left().unwrap();
        assert_eq!(middle_column.values(), vec!(&2, &5, &8));

        let first_column = middle_column.left().unwrap();
        assert_eq!(first_column.values(), vec!(&1, &4, &7));

        assert!(first_column.left().is_none());
    }

    #[test]
    fn column_right() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3],
                                        vec![4, 5, 6],
                                        vec![7, 8, 9]]);

        let first_column = grid.column(0);
        assert_eq!(first_column.values(), vec!(&1, &4, &7));

        let middle_column = first_column.right().unwrap();
        assert_eq!(middle_column.values(), vec!(&2, &5, &8));

        let last_column = middle_column.right().unwrap();
        assert_eq!(last_column.values(), vec!(&3, &6, &9));

        assert!(last_column.right().is_none());
    }
}