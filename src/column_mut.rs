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
use crate::column::Column;
use crate::iterator_column::IteratorColumn;
use crate::coord;

/// A mutable view onto a column of a grid
///
/// This structure is an **mutable** view into a column of a grid and its
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
/// column on the left or on the right with the `left()` and `right()` methods.
/// You can also conveniently iterate over its elements with the `iterator()`
/// method which returns an efficient iterator.
///
/// Unlike their **immutable** counter-part, there are additional operations you
/// can do such as reversing the elements of the column, or rotate them to the
/// top or the bottom.
///
/// # Examples
///
/// Iterating over the elements of a column.
///
/// ```
/// # use ingrid::{Grid, GridIterator};
/// #
/// let mut grid = Grid::from_rows(vec![vec![1, 2],
///                                     vec![3, 4],
///                                     vec![5, 6]]);
///
/// let column = grid.column_mut(0);
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
/// let mut grid = Grid::from_rows(vec![vec![1, 2],
///                                     vec![3, 4],
///                                     vec![5, 6]]);
///
/// println!("First element of first column is {}", grid.column_mut(0)[0]);
/// println!("Last element of last column is {}", grid.column_mut(1)[2]);
/// ```
///
#[derive(Debug, Eq, PartialEq)]
pub struct ColumnMut<'a, T> {
    /// A reference to its grid.
    pub grid: &'a mut Grid<T>,

    /// The index of the column.
    pub index: usize
}

impl<'a, T: Clone> ColumnMut<'a, T> {

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
    /// let mut grid = Grid::with_size(size!(2, 3), 42);
    ///
    /// assert_eq!(grid.column_mut(0).length(), 3);
    /// assert_eq!(grid.column_mut(1).length(), 3);
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4],
    ///                                     vec![5, 6]]);
    ///
    /// let column = grid.column_mut(1);
    /// assert_eq!(column.value(0), &2);
    /// assert_eq!(column.value(1), &4);
    /// assert_eq!(column.value(2), &6);
    /// column.value(3); // It panics here !
    /// ```
    ///
    pub fn value(&self, index: usize) -> &T {
        self.grid.value(coord!(self.index, index))
    }

    /// Returns a mutable reference to an element of the column.
    ///
    /// This method returns a mutable reference to an element of the column from
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 0],
    ///                                     vec![5, 6]]);
    ///
    /// let mut column = grid.column_mut(1);
    /// *column.value_mut(1) = 4;
    ///
    /// assert_eq!(column.value(0), &2);
    /// assert_eq!(column.value(1), &4);
    /// assert_eq!(column.value(2), &6);
    ///
    /// column.value(3); // It panics here !
    /// ```
    ///
    pub fn value_mut(&mut self, index: usize) -> &mut T {
        self.grid.value_mut(coord!(self.index, index))
    }

    /// Replace an element of the column.
    ///
    /// This method replaces the value of an element of the column from its
    /// index and a new value, effectively dropping the previous value.
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 0],
    ///                                     vec![5, 6]]);
    ///
    /// let mut column = grid.column_mut(1);
    /// column.set_value(1, 4);
    ///
    /// assert_eq!(column.value(0), &2);
    /// assert_eq!(column.value(1), &4);
    /// assert_eq!(column.value(2), &6);
    ///
    /// column.set_value(3, 42); // It panics here !
    /// ```
    ///
    pub fn set_value(&mut self, index: usize, value: T) {
        self.grid.set_value(coord!(self.index, index), value);
    }

    /// Swap two elements of the column.
    ///
    /// This method swaps two elements of the column from their index.
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 6],
    ///                                     vec![3, 4],
    ///                                     vec![5, 2]]);
    ///
    /// let mut column = grid.column_mut(1);
    /// column.swap_value(0, 2);
    ///
    /// assert_eq!(column.value(0), &2);
    /// assert_eq!(column.value(1), &4);
    /// assert_eq!(column.value(2), &6);
    ///
    /// column.swap_value(1, 3); // It panics here !
    /// ```
    ///
    pub fn swap_value(&mut self, a: usize, b: usize) {
        self.grid.swap_value(coord!(self.index, a), coord!(self.index, b));
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    ///
    /// assert_eq!(grid.column_mut(0).values(), vec![&1, &3]);
    /// assert_eq!(grid.column_mut(1).values(), vec![&2, &4]);
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4],
    ///                                     vec![5, 6]]);
    ///
    /// // The first element of the second column is 2.
    /// let column = grid.column_mut(1);
    /// assert_eq!(column.top(), &2);
    /// ```
    ///
    pub fn top(&self) -> &T {
        self.grid.value(coord!(self.index, 0))
    }

    /// Returns a mutable reference to the first element of the column.
    ///
    /// This method returns a mutable reference to the first element of the
    /// column. It's equivalent to retrieving the element with index `0`.
    ///
    /// Note that there is always a first element or the grid would have no
    /// size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 0],
    ///                                     vec![3, 4],
    ///                                     vec![5, 6]]);
    ///
    /// // The first element of the second column is 0 but we can change it to 2.
    /// let mut column = grid.column_mut(1);
    /// *column.top_mut() = 2;
    ///
    /// assert_eq!(column.top(), &2);
    /// ```
    ///
    pub fn top_mut(&mut self) -> &mut T {
        self.grid.value_mut(coord!(self.index, 0))
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4],
    ///                                     vec![5, 6]]);
    ///
    /// // The last element of the second column is 6.
    /// let column = grid.column_mut(1);
    /// assert_eq!(column.bottom(), &6);
    /// ```
    ///
    pub fn bottom(&self) -> &T {
        self.grid.value(coord!(self.index, self.grid.size().height-1))
    }

    /// Returns a mutable reference to the last element of the column.
    ///
    /// This method returns a mutable reference to the last element of the
    /// column. It's equivalent to retrieving the element with index
    /// `length() -1`.
    ///
    /// Note that there is always a last element or the grid would have no
    /// size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4],
    ///                                     vec![5, 0]]);
    ///
    /// // The last element of the second column is 0 but we can change it to 6.
    /// let mut column = grid.column_mut(1);
    /// *column.bottom_mut() = 6;
    ///
    /// assert_eq!(column.bottom_mut(), &6);
    /// ```
    ///
    pub fn bottom_mut(&mut self) -> &mut T {
        self.grid.value_mut(coord!(self.index, self.grid.size().height-1))
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 42],
    ///                                     vec![3, 42],
    ///                                     vec![5, 42]]);
    ///
    /// // Check if all elements of the column have value 42.
    /// assert_eq!(grid.column_mut(0).iterator().all(|item| *item == 42), false);
    /// assert_eq!(grid.column_mut(1).iterator().all(|item| *item == 42), true);
    /// ```
    ///
    pub fn iterator(&'a self) -> IteratorColumn<'a, T> {
        IteratorColumn::new(self.grid.column(self.index))
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// let second_column = grid.column_mut(1);
    /// let first_column = second_column.left().unwrap();
    /// assert!(first_column.left().is_none()); // There is no column on the left.
    /// ```
    ///
    pub fn left(&'a self) -> Option<Column<'a, T>> {
        match self.index {
            0 => None,
            index => Some(self.grid.column(index - 1))
        }
    }

    /// Returns the mutable column on the left.
    ///
    /// This method returns the mutable column on the left of this row, or
    /// `None` if this is already the column at the very left of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// let mut second_column = grid.column_mut(1);
    /// let mut first_column = second_column.left_mut().unwrap();
    /// assert!(first_column.left_mut().is_none()); // There is no column on the left.
    /// ```
    ///
    pub fn left_mut(&'a mut self) -> Option<ColumnMut<'a, T>> {
        match self.index {
            0 => None,
            index => Some(self.grid.column_mut(index - 1))
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// let mut first_column = grid.column_mut(0);
    /// let second_column = first_column.right().unwrap();
    /// assert!(second_column.right().is_none()); // There is no column on the right.
    /// ```
    ///
    pub fn right(&'a self) -> Option<Column<'a, T>> {
        if self.index == self.grid.size().width - 1 {
            None
        }
        else {
            // rework this
            let rigth_column_index: usize = (self.index + 1) as usize;
            Some(self.grid.column(rigth_column_index)) // remove integer conversation
        }
    }

    /// Returns the mutable column on the right.
    ///
    /// This method returns the mutable column on the right of this column, or
    /// `None` if this is already the column at the very right of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// let mut first_column = grid.column_mut(0);
    /// let mut second_column = first_column.right_mut().unwrap();
    /// assert!(second_column.right_mut().is_none()); // There is no column on the right.
    /// ```
    ///
    pub fn right_mut(&'a mut self) -> Option<ColumnMut<'a, T>> {
        if self.index == self.grid.size().width - 1 {
            None
        }
        else {
            // rework this
            let rigth_column_index: usize = (self.index + 1) as usize;
            Some(self.grid.column_mut(rigth_column_index)) // remove integer conversation
        }
    }

    /// Reverse the order of the elements.
    ///
    /// This method reverses the order of the elements in the column, in place.
    ///
    /// Note that it's similar to the `reverse()` method of the slice primitive
    /// type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 6],
    ///                                     vec![3, 4],
    ///                                     vec![5, 2]]);
    ///
    /// let mut column = grid.column_mut(1);
    /// column.reverse();
    ///
    /// assert_eq!(column.value(0), &2);
    /// assert_eq!(column.value(1), &4);
    /// assert_eq!(column.value(2), &6);
    /// ```
    ///
    pub fn reverse(&mut self) {
        let mut index: usize = 0;
        let length = self.length();

        while index < length / 2 {
            self.swap(index, length - index - 1);
            index += 1;
        }
    }

    /// Rotate elements to the top.
    ///
    /// This method rotates the column in-place such that the elements are moved
    /// a given `number` of times to the top. The elements that goes out of the
    /// column are added back to the bottom of the column.
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
    /// column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4],
    ///                                     vec![5, 6]]);
    ///
    /// let mut column = grid.column_mut(1);
    /// column.rotate_top(1);
    ///
    /// assert_eq!(column.value(0), &4);
    /// assert_eq!(column.value(1), &6);
    /// assert_eq!(column.value(2), &2);
    /// ```
    ///
    pub fn rotate_top(&mut self, number: usize) {
        assert!(number <= self.length());

        let length = self.length();

        let mut i = number;
        for j in 0..length-1 {
            self.swap_value(i % length, j);
            i += 1
        }
    }

    /// Rotate elements to the bottom.
    ///
    /// This method rotates the column in-place such that the elements are moved
    /// a given `number` of times to the bottom. The elements that goes out of
    /// the column are added back to the top of the column.
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
    /// column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4],
    ///                                     vec![5, 6]]);
    ///
    /// let mut column = grid.column_mut(1);
    /// column.rotate_bottom(1);
    ///
    /// assert_eq!(column.value(0), &6);
    /// assert_eq!(column.value(1), &2);
    /// assert_eq!(column.value(2), &4);
    /// ```
    ///
    pub fn rotate_bottom(&mut self, number: usize) {
        // assert!(number <= self.length());

        let length = self.length();
        let mut i = number + length;

        for j in (1..length).rev() {
            let foo = i % length;
            let bar = j;

            self.swap_value(foo, bar);
            i -= 1;
        }
    }

    /// Swap two elements in the column.
    ///
    /// This method swaps two elements in the column.
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 6],
    ///                                     vec![3, 4],
    ///                                     vec![5, 2]]);
    ///
    /// let mut column = grid.column_mut(1);
    /// column.swap(0, 2);
    ///
    /// assert_eq!(column.value(0), &2);
    /// assert_eq!(column.value(1), &4);
    /// assert_eq!(column.value(2), &6);
    /// ```
    ///
    pub fn swap(&mut self, a: usize, b: usize) {
        self.grid.swap_value(coord!(self.index, a), coord!(self.index, b));
    }
}

impl<'a, T: Clone> Index<usize> for ColumnMut<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.value(index)
    }
}

impl<'a, T: Clone> IndexMut<usize> for ColumnMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.value_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_length() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        assert_eq!(grid.column_mut(0).length(), 3);
        assert_eq!(grid.column_mut(1).length(), 3);
        assert_eq!(grid.column_mut(2).length(), 3);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn column_value() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4]]);

        let column = grid.column_mut(0);
        assert_eq!(column.value(0), &1);
        assert_eq!(column.value(1), &3);

        let column = grid.column_mut(1);
        assert_eq!(column.value(0), &2);
        assert_eq!(column.value(1), &4);

        column.value(2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn column_value_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 0]]);
        *grid.column_mut(1).value_mut(1) = 4;

        let mut column = grid.column_mut(0);
        assert_eq!(column.value_mut(0), &1);
        assert_eq!(column.value_mut(1), &3);

        let mut column = grid.column_mut(1);
        assert_eq!(column.value_mut(0), &2);
        assert_eq!(column.value_mut(1), &4);

        column.value_mut(2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn column_set_value() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 0]]);

        let mut column = grid.column_mut(1);
        column.set_value(1, 4);

        assert_eq!(grid.value(coord!(0, 0)), &1);
        assert_eq!(grid.value(coord!(1, 0)), &2);
        assert_eq!(grid.value(coord!(0, 1)), &3);
        assert_eq!(grid.value(coord!(1, 1)), &4);

        let mut column = grid.column_mut(0);
        column.set_value(2, 5);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn column_swap_value() {
        let mut grid = Grid::from_rows(vec![vec![1, 4],
                                            vec![3, 2]]);

        let mut column = grid.column_mut(1);
        column.swap_value(0, 1);

        assert_eq!(grid.value(coord!(0, 0)), &1);
        assert_eq!(grid.value(coord!(1, 0)), &2);
        assert_eq!(grid.value(coord!(0, 1)), &3);
        assert_eq!(grid.value(coord!(1, 1)), &4);

        let mut column = grid.column_mut(0);
        column.swap_value(1, 2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn column_index() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4]]);

        let column = grid.column_mut(0);
        assert_eq!(column[0], 1);
        assert_eq!(column[1], 3);

        let column = grid.column_mut(1);
        assert_eq!(column[0], 2);
        assert_eq!(column[1], 4);

        column[2];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn column_index_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 0]]);

        let mut column = grid.column_mut(1);
        column[1] = 4;

        let mut column = grid.column_mut(0);
        assert_eq!(column[0], 1);
        assert_eq!(column[1], 3);

        let mut column = grid.column_mut(1);
        assert_eq!(column[0], 2);
        assert_eq!(column[1], 4);

        column[2];
    }

    #[test]
    fn column_top() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        assert_eq!(grid.column_mut(0).top(), &1);
        assert_eq!(grid.column_mut(1).top(), &2);
        assert_eq!(grid.column_mut(2).top(), &3);
    }

    #[test]
    fn column_top_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 0, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);
        *grid.column_mut(1).top_mut() = 2;
        assert_eq!(grid.column_mut(0).top_mut(), &1);
        assert_eq!(grid.column_mut(1).top_mut(), &2);
        assert_eq!(grid.column_mut(2).top_mut(), &3);
    }

    #[test]
    fn column_bottom() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        assert_eq!(grid.column_mut(0).bottom(), &7);
        assert_eq!(grid.column_mut(1).bottom(), &8);
        assert_eq!(grid.column_mut(2).bottom(), &9);
    }

    #[test]
    fn column_bottom_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 0, 9]]);

        *grid.column_mut(1).bottom_mut() = 8;
        assert_eq!(grid.column_mut(0).bottom_mut(), &7);
        assert_eq!(grid.column_mut(1).bottom_mut(), &8);
        assert_eq!(grid.column_mut(2).bottom_mut(), &9);
    }

    #[test]
    fn column_iterator() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4],
                                            vec![5, 6]]);

        let column = grid.column_mut(0);
        let mut iterator = column.iterator();

        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), None);

        let column = grid.column_mut(1);
        let mut iterator = column.iterator();

        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn column_iterator_mut() {
    }

    #[test]
    fn column_left() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        let last_column = grid.column_mut(2);
        assert_eq!(last_column.values(), vec!(&3, &6, &9));

        let middle_column = last_column.left().unwrap();
        assert_eq!(middle_column.values(), vec!(&2, &5, &8));

        let first_column = middle_column.left().unwrap();
        assert_eq!(first_column.values(), vec!(&1, &4, &7));

        assert!(first_column.left().is_none());
    }

    #[test]
    fn column_left_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 0, 6],
                                            vec![7, 8, 9]]);

        let mut last_column = grid.column_mut(2);
        assert_eq!(last_column.values(), vec!(&3, &6, &9));

        let mut middle_column = last_column.left_mut().unwrap();
        middle_column[1] = 5;
        assert_eq!(middle_column.values(), vec!(&2, &5, &8));

        let mut first_column = middle_column.left_mut().unwrap();
        assert_eq!(first_column.values(), vec!(&1, &4, &7));

        assert!(first_column.left_mut().is_none());
    }

    #[test]
    fn column_right() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        let first_column = grid.column_mut(0);
        assert_eq!(first_column.values(), vec!(&1, &4, &7));

        let middle_column = first_column.right().unwrap();
        assert_eq!(middle_column.values(), vec!(&2, &5, &8));

        let last_column = middle_column.right().unwrap();
        assert_eq!(last_column.values(), vec!(&3, &6, &9));

        assert!(last_column.right().is_none());
    }

    #[test]
    fn column_right_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 0, 6],
                                            vec![7, 8, 9]]);

        let mut first_column = grid.column_mut(0);
        assert_eq!(first_column.values(), vec!(&1, &4, &7));

        let mut middle_column = first_column.right_mut().unwrap();
        middle_column[1] = 5;
        assert_eq!(middle_column.values(), vec!(&2, &5, &8));

        let mut last_column = middle_column.right_mut().unwrap();
        assert_eq!(last_column.values(), vec!(&3, &6, &9));

        assert!(last_column.right_mut().is_none());
    }

    #[test]
    fn column_reverse() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);
        grid.column_mut(0).reverse();
        assert_eq!(grid.column(0).values(), vec!(&7, &4, &1));
        assert_eq!(grid.column(1).values(), vec!(&2, &5, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(1).reverse();
        assert_eq!(grid.column(0).values(), vec!(&7, &4, &1));
        assert_eq!(grid.column(1).values(), vec!(&8, &5, &2));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(2).reverse();
        assert_eq!(grid.column(0).values(), vec!(&7, &4, &1));
        assert_eq!(grid.column(1).values(), vec!(&8, &5, &2));
        assert_eq!(grid.column(2).values(), vec!(&9, &6, &3));
    }

    #[test]
    fn column_rotate_top() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.column_mut(0).rotate_top(1);
        assert_eq!(grid.column(0).values(), vec!(&4, &7, &1));
        assert_eq!(grid.column(1).values(), vec!(&2, &5, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(0).rotate_top(1);
        assert_eq!(grid.column(0).values(), vec!(&7, &1, &4));
        assert_eq!(grid.column(1).values(), vec!(&2, &5, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(0).rotate_top(1);
        assert_eq!(grid.column(0).values(), vec!(&1, &4, &7));
        assert_eq!(grid.column(1).values(), vec!(&2, &5, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(1).rotate_top(2);
        assert_eq!(grid.column(0).values(), vec!(&1, &4, &7));
        assert_eq!(grid.column(1).values(), vec!(&5, &8, &2));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(2).rotate_top(0);
        assert_eq!(grid.column(0).values(), vec!(&1, &4, &7));
        assert_eq!(grid.column(1).values(), vec!(&5, &8, &2));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));
    }

    #[test]
    fn column_rotate_bottom() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.column_mut(0).rotate_bottom(1);
        assert_eq!(grid.column(0).values(), vec!(&7, &1, &4));
        assert_eq!(grid.column(1).values(), vec!(&2, &5, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(0).rotate_bottom(1);
        assert_eq!(grid.column(0).values(), vec!(&4, &7, &1));
        assert_eq!(grid.column(1).values(), vec!(&2, &5, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(0).rotate_bottom(1);
        assert_eq!(grid.column(0).values(), vec!(&1, &4, &7));
        assert_eq!(grid.column(1).values(), vec!(&2, &5, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(1).rotate_bottom(2);
        assert_eq!(grid.column(0).values(), vec!(&1, &4, &7));
        assert_eq!(grid.column(1).values(), vec!(&2, &5, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(2).rotate_bottom(0);
        assert_eq!(grid.column(0).values(), vec!(&1, &4, &7));
        assert_eq!(grid.column(1).values(), vec!(&2, &5, &8));
        assert_eq!(grid.column(2).values(), vec!(&9, &3, &6));
    }

    #[test]
    fn column_swap() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.column_mut(0).swap(0, 2);
        assert_eq!(grid.column(0).values(), vec!(&7, &4, &1));
        assert_eq!(grid.column(1).values(), vec!(&2, &5, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(1).swap(0, 1);
        assert_eq!(grid.column(0).values(), vec!(&7, &4, &1));
        assert_eq!(grid.column(1).values(), vec!(&5, &2, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &6, &9));

        grid.column_mut(2).swap(1, 2);
        assert_eq!(grid.column(0).values(), vec!(&7, &4, &1));
        assert_eq!(grid.column(1).values(), vec!(&5, &2, &8));
        assert_eq!(grid.column(2).values(), vec!(&3, &9, &6));
    }

    #[test]
    fn column_swap_with_slice() {
        // Not implemented yet.
    }
}