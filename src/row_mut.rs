// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::ops::{Index, IndexMut};
use crate::coordinate::Coordinate;
use crate::grid::Grid;
use crate::row::Row;
use crate::iterator_row::IteratorRow;
use crate::coord;

/// A mutable view onto a row of a grid
///
/// This structure is an **mutable** view into a row of a grid and its
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
/// Unlike their **immutable** counter-part, there are additional operations you
/// can do such as reversing the elements of the row, or rotate them to the left
/// or the right.
///
/// # Examples
///
/// Iterating over the elements of a row.
///
/// ```
/// # use ingrid::{Grid, GridIterator};
/// #
/// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
///                                     vec![4, 5, 6]]);
///
/// let row = grid.row_mut(0);
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
/// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
///                                     vec![4, 5, 6]]);
///
/// println!("First element of first row is {}", grid.row_mut(0)[0]);
/// println!("Last element of last row is {}", grid.row_mut(1)[2]);
/// ```
///
#[derive(Debug, Eq, PartialEq)]
pub struct RowMut<'a, T> {
    /// A reference to its grid.
    pub grid: &'a mut Grid<T>,

    /// The index of the row.
    pub index: usize
}

impl<'a, T: Clone> RowMut<'a, T> {

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
    /// let mut grid = Grid::with_size(size!(3, 2), 42);
    ///
    /// assert_eq!(grid.row_mut(0).length(), 3);
    /// assert_eq!(grid.row_mut(1).length(), 3);
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
    ///
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![4, 5, 6]]);
    ///
    /// let row = grid.row_mut(1);
    /// assert_eq!(row.value(0), &4);
    /// assert_eq!(row.value(1), &5);
    /// assert_eq!(row.value(2), &6);
    /// row.value(3); // It panics here !
    /// ```
    ///
    pub fn value(&self, index: usize) -> &T {
        self.grid.value(coord!(index, self.index))
    }

    /// Returns a mutable reference to an element of the row.
    ///
    /// This method returns a mutable reference to an element of the row from
    /// its index.
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![4, 0, 6]]);
    ///
    /// let mut row = grid.row_mut(1);
    /// *row.value_mut(1) = 5;
    ///
    /// assert_eq!(row.value(0), &4);
    /// assert_eq!(row.value(1), &5);
    /// assert_eq!(row.value(2), &6);
    ///
    /// row.value(3); // It panics here !
    /// ```
    ///
    pub fn value_mut(&mut self, index: usize) -> &mut T {
        self.grid.value_mut(coord!(index, self.index))
    }

    /// Replace an element of the row.
    ///
    /// This method replaces the value of an element of the row from its index
    /// and a new value, effectively dropping the previous value.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the element
    /// * `value` - New value of the element
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![4, 0, 6]]);
    ///
    /// let mut row = grid.row_mut(1);
    /// row.set_value(1, 5);
    ///
    /// assert_eq!(row.value(0), &4);
    /// assert_eq!(row.value(1), &5);
    /// assert_eq!(row.value(2), &6);
    ///
    /// row.set_value(3, 42); // It panics here !
    /// ```
    ///
    pub fn set_value(&mut self, index: usize, value: T) {
        self.grid.set_value(coord!(index, self.index), value);
    }

    /// Swap two elements of the row.
    ///
    /// This method swaps two elements of the row from their index.
    ///
    /// # Arguments
    ///
    /// * `a` - Index of one of the element to swap
    /// * `b` - Index of the other element to be swapped with
    ///
    /// # Panics
    ///
    /// It panics if the indexes are out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![6, 5, 4]]);
    ///
    /// let mut row = grid.row_mut(1);
    /// row.swap_value(0, 2);
    ///
    /// assert_eq!(row.value(0), &4);
    /// assert_eq!(row.value(1), &5);
    /// assert_eq!(row.value(2), &6);
    ///
    /// row.swap_value(1, 3); // It panics here !
    /// ```
    ///
    pub fn swap_value(&mut self, a: usize, b: usize) {
        self.grid.swap_value(coord!(a, self.index), coord!(b, self.index));
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// assert_eq!(grid.row_mut(0).values(), vec![&1, &2]);
    /// assert_eq!(grid.row_mut(1).values(), vec![&3, &4]);
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![4, 5, 6]]);
    ///
    /// // The first element of the second row is 4.
    /// let row = grid.row_mut(1);
    /// assert_eq!(row.left(), &4);
    /// ```
    ///
    pub fn left(&self) -> &T {
        self.grid.value(coord!(0, self.index))
    }

    /// Returns a mutable reference to the first element of the row.
    ///
    /// This method returns a mutable reference to the first element of the
    /// row. It's equivalent to retrieving the element with index `0`.
    ///
    /// Note that there is always a first element or the grid would have no
    /// size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![0, 5, 6]]);
    ///
    /// // The first element of the second row is 0 but we can change it to 4.
    /// let mut row = grid.row_mut(1);
    /// *row.left_mut() = 4;
    ///
    /// assert_eq!(row.left(), &4);
    /// ```
    ///
    pub fn left_mut(&mut self) -> &mut T {
        self.grid.value_mut(coord!(0, self.index))
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![4, 5, 6]]);
    ///
    /// // The last element of the second row is 6.
    /// let row = grid.row_mut(1);
    /// assert_eq!(row.right(), &6);
    /// ```
    ///
    pub fn right(&self) -> &T {
        self.grid.value(coord!(self.grid.size().width-1, self.index))
    }

    /// Returns a mutable reference to the last element of the row.
    ///
    /// This method returns a mutable reference to the last element of the row.
    /// It's equivalent to retrieving the element with index `length() -1`.
    ///
    /// Note that there is always a last element or the grid would have no
    /// size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![4, 5, 0]]);
    ///
    /// // The last element of the second row is 0 but we can change it to 6.
    /// let mut row = grid.row_mut(1);
    /// *row.right_mut() = 6;
    ///
    /// assert_eq!(row.right_mut(), &6);
    /// ```
    ///
    pub fn right_mut(&mut self) -> &mut T {
        self.grid.value_mut(coord!(self.grid.size().width-1, self.index))
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
    /// let mut grid = Grid::from_rows(vec![vec![ 1,  2,  3],
    ///                                     vec![42, 42, 42]]);
    ///
    /// // Check if all elements of the row have value 42.
    /// assert_eq!(grid.row_mut(0).iterator().all(|item| *item == 42), false);
    /// assert_eq!(grid.row_mut(1).iterator().all(|item| *item == 42), true);
    /// ```
    ///
    pub fn iterator(&'a self) -> IteratorRow<'a, T> {
        IteratorRow::new(self.grid.row(self.index))
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// let second_row = grid.row_mut(1);
    /// let first_row = second_row.top().unwrap();
    /// assert!(first_row.top().is_none()); // There is no row above.
    /// ```
    ///
    pub fn top(&'a self) -> Option<Row<'a, T>> {
        match self.index {
            0 => None,
            index => Some(self.grid.row(index - 1))
        }
    }

    /// Returns the mutable row above.
    ///
    /// This method returns the mutable row above this row, or `None` if this is
    /// already the row at the very top of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// let mut second_row = grid.row_mut(1);
    /// let mut first_row = second_row.top_mut().unwrap();
    /// assert!(first_row.top_mut().is_none()); // There is no row above.
    /// ```
    ///
    pub fn top_mut(&'a mut self) -> Option<RowMut<'a, T>> {
        match self.index {
            0 => None,
            index => Some(self.grid.row_mut(index - 1))
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// let mut first_row = grid.row_mut(0);
    /// let second_row = first_row.bottom().unwrap();
    /// assert!(second_row.bottom().is_none()); // There is no row below.
    /// ```
    ///
    pub fn bottom(&'a self) -> Option<Row<'a, T>> {
        // rework this to use match syntax
        if self.index == self.length() -1 {
            None
        }
        else {
            Some(self.grid.row(self.index + 1))
        }
    }

    /// Returns the mutable row below.
    ///
    /// This method returns the mutable row below this row, or `None` if this is
    /// already the row at the very bottom of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// let mut first_row = grid.row_mut(0);
    /// let mut second_row = first_row.bottom_mut().unwrap();
    /// assert!(second_row.bottom_mut().is_none()); // There is no row below.
    /// ```
    ///
    pub fn bottom_mut(&'a mut self) -> Option<RowMut<'a, T>> {
        // rework this to use match syntax
        if self.index == self.length() -1 {
            None
        }
        else {
            Some(self.grid.row_mut(self.index + 1))
        }
    }

    /// Reverse the order of the elements.
    ///
    /// This method reverses the order of the elements in the row, in place.
    ///
    /// Note that it's similar to the `reverse()` method of the slice primitive
    /// type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![6, 5, 4]]);
    ///
    /// let mut row = grid.row_mut(1);
    /// row.reverse();
    ///
    /// assert_eq!(row.value(0), &4);
    /// assert_eq!(row.value(1), &5);
    /// assert_eq!(row.value(2), &6);
    /// ```
    ///
    pub fn reverse(&mut self) {
        self.grid.row_slice(self.index).reverse();
    }

    /// Rotate elements to the left.
    ///
    /// This method rotates the row in-place such that the elements are moved
    /// a given `number` of times to the left. The elements that goes out of the
    /// row are added back to the right of the row.
    ///
    /// Note that it's similar to the `rotate_left()` method of the slice
    /// primitive type.
    ///
    /// # Arguments
    ///
    /// * number - The number of times elements are rotated
    ///
    /// # Panics
    ///
    /// This function will panic if `number` is greater than the length of the
    /// row.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![4, 5, 6]]);
    ///
    /// let mut row = grid.row_mut(1);
    /// row.rotate_left(1);
    ///
    /// assert_eq!(row.value(0), &5);
    /// assert_eq!(row.value(1), &6);
    /// assert_eq!(row.value(2), &4);
    /// ```
    ///
    pub fn rotate_left(&mut self, number: usize) {
        self.grid.row_slice(self.index).rotate_left(number);
    }

    /// Rotate elements to the right.
    ///
    /// This method rotates the row in-place such that the elements are moved
    /// a given `number` of times to the right. The elements that goes out of
    /// the row are added back to the left of the row.
    ///
    /// Note that it's similar to the `rotate_right()` method of the slice
    /// primitive type.
    ///
    /// # Arguments
    ///
    /// * number - The number of times elements are rotated
    ///
    /// # Panics
    ///
    /// This function will panic if `number` is greater than the length of the
    /// row.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![4, 5, 6]]);
    ///
    /// let mut row = grid.row_mut(1);
    /// row.rotate_right(1);
    ///
    /// assert_eq!(row.value(0), &6);
    /// assert_eq!(row.value(1), &4);
    /// assert_eq!(row.value(2), &5);
    /// ```
    ///
    pub fn rotate_right(&mut self, number: usize) {
        self.grid.row_slice(self.index).rotate_right(number);
    }

    /// Swap two elements in the row.
    ///
    /// This method swaps two elements in the row.
    ///
    /// Note that it's similar to the `swap()` method of the slice primitive
    /// type.
    ///
    /// # Arguments
    ///
    /// * a - The index of the first element
    /// * b - The index of the second element
    ///
    /// # Panics
    ///
    /// It panics if `a` or `b` are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![6, 5, 4]]);
    ///
    /// let mut row = grid.row_mut(1);
    /// row.swap(0, 2);
    ///
    /// assert_eq!(row.value(0), &4);
    /// assert_eq!(row.value(1), &5);
    /// assert_eq!(row.value(2), &6);
    /// ```
    ///
    pub fn swap(&mut self, a: usize, b: usize) {
        self.grid.row_slice(self.index).swap(a, b);
    }
}

impl<'a, T: Clone> Index<usize> for RowMut<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.value(index)
    }
}

impl<'a, T: Clone> IndexMut<usize> for RowMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.value_mut(index)
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
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4]]);

        let row = grid.row_mut(0);
        assert_eq!(row.value(0), &1);
        assert_eq!(row.value(1), &2);

        let row = grid.row_mut(1);
        assert_eq!(row.value(0), &3);
        assert_eq!(row.value(1), &4);

        row.value(2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_value_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 0]]);

        *grid.row_mut(1).value_mut(1) = 4;

        let mut row = grid.row_mut(0);
        assert_eq!(row.value_mut(0), &1);
        assert_eq!(row.value_mut(1), &2);

        let mut row = grid.row_mut(1);
        assert_eq!(row.value_mut(0), &3);
        assert_eq!(row.value_mut(1), &4);

        row.value_mut(2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_set_value() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 0]]);

        let mut row = grid.row_mut(1);
        row.set_value(1, 4);

        assert_eq!(grid.value(coord!(0, 0)), &1);
        assert_eq!(grid.value(coord!(1, 0)), &2);
        assert_eq!(grid.value(coord!(0, 1)), &3);
        assert_eq!(grid.value(coord!(1, 1)), &4);

        let mut row = grid.row_mut(0);
        row.set_value(2, 5);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_swap_value() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![4, 3]]);

        let mut row = grid.row_mut(1);
        row.swap_value(0, 1);

        assert_eq!(grid.value(coord!(0, 0)), &1);
        assert_eq!(grid.value(coord!(1, 0)), &2);
        assert_eq!(grid.value(coord!(0, 1)), &3);
        assert_eq!(grid.value(coord!(1, 1)), &4);

        let mut row = grid.row_mut(0);
        row.swap_value(1, 2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_index() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4]]);

        let row = grid.row_mut(0);
        assert_eq!(row[0], 1);
        assert_eq!(row[1], 2);

        let row = grid.row_mut(1);
        assert_eq!(row[0], 3);
        assert_eq!(row[1], 4);

        row[2];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_index_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 0]]);

        let mut row = grid.row_mut(1);
        row[1] = 4;

        let mut row = grid.row_mut(0);
        assert_eq!(row[0], 1);
        assert_eq!(row[1], 2);

        let mut row = grid.row_mut(1);
        assert_eq!(row[0], 3);
        assert_eq!(row[1], 4);

        row[2];
    }

    #[test]
    fn row_left() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        assert_eq!(grid.row_mut(0).left(), &1);
        assert_eq!(grid.row_mut(1).left(), &4);
        assert_eq!(grid.row_mut(2).left(), &7);
    }

    #[test]
    fn row_left_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![0, 5, 6],
                                            vec![7, 8, 9]]);

        *grid.row_mut(1).left_mut() = 4;
        assert_eq!(grid.row_mut(0).left_mut(), &1);
        assert_eq!(grid.row_mut(1).left_mut(), &4);
        assert_eq!(grid.row_mut(2).left_mut(), &7);
    }

    #[test]
    fn row_right() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        assert_eq!(grid.row_mut(0).right(), &3);
        assert_eq!(grid.row_mut(1).right(), &6);
        assert_eq!(grid.row_mut(2).right(), &9);
    }

    #[test]
    fn row_right_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 0],
                                            vec![7, 8, 9]]);

        *grid.row_mut(1).right_mut() = 6;
        assert_eq!(grid.row_mut(0).right_mut(), &3);
        assert_eq!(grid.row_mut(1).right_mut(), &6);
        assert_eq!(grid.row_mut(2).right_mut(), &9);
    }

    #[test]
    fn row_iterator() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6]]);

        let row = grid.row_mut(0);
        let mut iterator = row.iterator();

        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), None);

        let row = grid.row_mut(1);
        let mut iterator = row.iterator();

        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn row_top() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        let last_row = grid.row_mut(2);
        assert_eq!(last_row.values(), vec!(&7, &8, &9));

        let middle_row = last_row.top().unwrap();
        assert_eq!(middle_row.values(), vec!(&4, &5, &6));

        let first_row = middle_row.top().unwrap();
        assert_eq!(first_row.values(), vec!(&1, &2, &3));

        assert!(first_row.top().is_none());
    }

    #[test]
    fn row_top_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 0, 6],
                                            vec![7, 8, 9]]);

        let mut last_row = grid.row_mut(2);
        assert_eq!(last_row.values(), vec!(&7, &8, &9));

        let mut middle_row = last_row.top_mut().unwrap();
        middle_row[1] = 5;
        assert_eq!(middle_row.values(), vec!(&4, &5, &6));

        let mut first_row = middle_row.top_mut().unwrap();
        assert_eq!(first_row.values(), vec!(&1, &2, &3));

        assert!(first_row.top_mut().is_none());
    }

    #[test]
    fn row_bottom() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        let first_row = grid.row_mut(0);
        assert_eq!(first_row.values(), vec!(&1, &2, &3));

        let middle_row = first_row.bottom().unwrap();
        assert_eq!(middle_row.values(), vec!(&4, &5, &6));

        let last_row = middle_row.bottom().unwrap();
        assert_eq!(last_row.values(), vec!(&7, &8, &9));

        assert!(last_row.bottom().is_none());
    }

    #[test]
    fn row_bottom_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 0, 6],
                                            vec![7, 8, 9]]);

        let mut first_row = grid.row_mut(0);
        assert_eq!(first_row.values(), vec!(&1, &2, &3));

        let mut middle_row = first_row.bottom_mut().unwrap();
        middle_row[1] = 5;
        assert_eq!(middle_row.values(), vec!(&4, &5, &6));

        let mut last_row = middle_row.bottom_mut().unwrap();
        assert_eq!(last_row.values(), vec!(&7, &8, &9));

        assert!(last_row.bottom_mut().is_none());
    }

    #[test]
    fn row_reverse() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.row_mut(0).reverse();
        assert_eq!(grid.row(0).values(), vec!(&3, &2, &1));
        assert_eq!(grid.row(1).values(), vec!(&4, &5, &6));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(1).reverse();
        assert_eq!(grid.row(0).values(), vec!(&3, &2, &1));
        assert_eq!(grid.row(1).values(), vec!(&6, &5, &4));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(2).reverse();
        assert_eq!(grid.row(0).values(), vec!(&3, &2, &1));
        assert_eq!(grid.row(1).values(), vec!(&6, &5, &4));
        assert_eq!(grid.row(2).values(), vec!(&9, &8, &7));
    }

    #[test]
    fn row_rotate_left() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.row_mut(0).rotate_left(1);
        assert_eq!(grid.row(0).values(), vec!(&2, &3, &1));
        assert_eq!(grid.row(1).values(), vec!(&4, &5, &6));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(0).rotate_left(1);
        assert_eq!(grid.row(0).values(), vec!(&3, &1, &2));
        assert_eq!(grid.row(1).values(), vec!(&4, &5, &6));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(0).rotate_left(1);
        assert_eq!(grid.row(0).values(), vec!(&1, &2, &3));
        assert_eq!(grid.row(1).values(), vec!(&4, &5, &6));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(1).rotate_left(2);
        assert_eq!(grid.row(0).values(), vec!(&1, &2, &3));
        assert_eq!(grid.row(1).values(), vec!(&6, &4, &5));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(2).rotate_left(0);
        assert_eq!(grid.row(0).values(), vec!(&1, &2, &3));
        assert_eq!(grid.row(1).values(), vec!(&6, &4, &5));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));
    }

    #[test]
    fn row_rotate_right() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.row_mut(0).rotate_right(1);
        assert_eq!(grid.row(0).values(), vec!(&3, &1, &2));
        assert_eq!(grid.row(1).values(), vec!(&4, &5, &6));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(0).rotate_right(1);
        assert_eq!(grid.row(0).values(), vec!(&2, &3, &1));
        assert_eq!(grid.row(1).values(), vec!(&4, &5, &6));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(0).rotate_right(1);
        assert_eq!(grid.row(0).values(), vec!(&1, &2, &3));
        assert_eq!(grid.row(1).values(), vec!(&4, &5, &6));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(1).rotate_right(2);
        assert_eq!(grid.row(0).values(), vec!(&1, &2, &3));
        assert_eq!(grid.row(1).values(), vec!(&5, &6, &4));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(2).rotate_right(0);
        assert_eq!(grid.row(0).values(), vec!(&1, &2, &3));
        assert_eq!(grid.row(1).values(), vec!(&5, &6, &4));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));
    }

    #[test]
    fn row_swap() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.row_mut(0).swap(0, 2);
        assert_eq!(grid.row(0).values(), vec!(&3, &2, &1));
        assert_eq!(grid.row(1).values(), vec!(&4, &5, &6));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(1).swap(0, 1);
        assert_eq!(grid.row(0).values(), vec!(&3, &2, &1));
        assert_eq!(grid.row(1).values(), vec!(&5, &4, &6));
        assert_eq!(grid.row(2).values(), vec!(&7, &8, &9));

        grid.row_mut(2).swap(1, 2);
        assert_eq!(grid.row(0).values(), vec!(&3, &2, &1));
        assert_eq!(grid.row(1).values(), vec!(&5, &4, &6));
        assert_eq!(grid.row(2).values(), vec!(&7, &9, &8));
    }
}