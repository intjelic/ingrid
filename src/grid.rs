// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::ops::{Index, IndexMut};
use std::vec::Vec;
use crate::coordinate::Coordinate;
use crate::size::Size;
use crate::row::Row;
use crate::row_mut::RowMut;
use crate::column::Column;
use crate::column_mut::ColumnMut;
use crate::iterator_grid::IteratorGrid;
use crate::size;

/// A dynamic two-dimensional array
///
/// This structure defines a dynamic two-dimensional array named **grid**. Grids
/// can grow and shrink, and its elements are indexed with coordinates. The grid
/// is implemented from left-to-right and top-to-bottom, therefore the top-left
/// element is index (0, 0).
///
/// Elements can be accessed directly with `value()` and its related methods or
/// indirectly via `row()` and `column()` and their related methods.
///
/// To iterate over the elements of the grid, use the `iterator()` method, or
/// get an iterator from an intermediary row or column instead.
///
/// Some common operations on grids are also available such as flipping the
/// values on one of the two axis, or rotating to the left or right.
///
/// Just like regular vectors, grids also have a capacity indicating the actual
/// allocated memory on each axis and allows you to control it with with the
/// `reserve()` method.
///
/// # Examples
///
/// Creating and resizing a grid.
///
/// ```
/// # use ingrid::{Coordinate, Size, Grid, coord, size};
/// #
/// // Create a 2x2 grid filled with value 0.
/// let mut grid = Grid::with_size(size!(2, 2), 0);
///
/// // Increase the grid size by one, and set the new values at 42.
/// grid.resize(size!(3, 3), 42);
/// ```
///
/// Iterating over the elements of a grid.
///
/// ```
/// # use ingrid::Grid;
/// #
/// let mut grid = Grid::from_rows(vec![vec![1, 2],
///                                     vec![3, 4]]);
///
/// let mut iterator = grid.iterator();
/// assert_eq!(iterator.next(), Some(&1));
/// assert_eq!(iterator.next(), Some(&2));
/// assert_eq!(iterator.next(), Some(&3));
/// assert_eq!(iterator.next(), Some(&4));
/// assert_eq!(iterator.next(), None);
/// ```
///
/// Indexing the grid.
///
/// ```
/// # use ingrid::{Coordinate, Size, Grid, coord, size};
/// #
/// let mut grid = Grid::with_size(size!(2, 2), 0);
///
/// grid[coord!(0, 0)] = 1;
/// grid[coord!(1, 0)] = 2;
/// grid[coord!(0, 1)] = 3;
/// grid[coord!(1, 1)] = 4;
/// ```
///
/// Inserting and removing rows and columns.
///
/// ```
/// # use ingrid::{Size, Grid, size};
/// #
/// let mut grid = Grid::with_size(size!(1, 1), 1);
///
/// grid.insert_column(1, vec![2]);
/// grid.insert_row(1, vec![3, 4]);
/// ```
///
#[derive(Debug, Eq, PartialEq)]
pub struct Grid<T> {
    size: Size,
    rows: Vec<Vec<T>>,
    row_capacity: usize
}

impl<T: Clone> Grid<T> {

    /// Create an empty grid
    ///
    /// This function creates an empty grid; a grid which has no width nor
    /// height, and thus contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let grid = Grid::<char>::new();
    /// assert_eq!(grid.size(), size!(0, 0));
    /// ```
    ///
    pub fn new() -> Grid<T> {
        Grid::<T> {
            size: Size::new(0, 0),
            rows: Vec::<Vec<T>>::with_capacity(0),
            row_capacity: 0
        }
    }

    /// Create a grid from a given size and value.
    ///
    /// This function creates a grid from a given size and value. The grid is
    /// initialized with the given value which is cloned to fill the entire
    /// grid.
    ///
    /// # Arguments
    ///
    /// * `size`  - The size of the grid.
    /// * `value` - The value to initialize the grid with.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Coordinate, Size, Grid, coord, size};
    /// #
    /// let grid = Grid::with_size(size!(2, 2), 42);
    ///
    /// assert_eq!(grid.value(coord!(0, 0)), &42);
    /// assert_eq!(grid.value(coord!(1, 0)), &42);
    /// assert_eq!(grid.value(coord!(0, 1)), &42);
    /// assert_eq!(grid.value(coord!(1, 1)), &42);
    /// ```
    ///
    pub fn with_size(size: Size, value: T) -> Grid<T> {
        let mut rows = Vec::<Vec<T>>::with_capacity(size.height);
        rows.resize_with(size.height, || {
            let mut row = Vec::<T>::with_capacity(size.width);
            row.resize(size.width, value.clone());

            row
        });

        Grid::<T> { size, rows, row_capacity: size.width }
    }

    /// Create a new grid with the specified capacity
    ///
    /// This function creates a grid with the specified capacity. The grid will
    /// be able to hold exactly the number of elements specified by the
    /// specified capacity without reallocating. If capacity is (0, 0), the the
    /// grid will not allocate.
    ///
    /// It is important to note that although the returned grid has the capacity
    /// specified, the grid will have a zero size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let mut grid = Grid::with_capacity(size!(2, 3));
    ///
    /// grid.resize(size!(2, 3), 42); // No allocation occurs here.
    /// grid.resize(size!(3, 3), 42); // Allocation occurs here.
    /// ```
    ///
    pub fn with_capacity(capacity: Size) -> Grid<T> {
        let mut rows = Vec::<Vec<T>>::with_capacity(capacity.height);
        rows.resize_with(capacity.height, || Vec::<T>::with_capacity(capacity.width));

        Grid::<T> {
            size: Size::new(0, 0),
            rows: rows,
            row_capacity: capacity.width
        }
    }

    /// Create a grid from rows
    ///
    /// This function creates a grid from a list of vectors denoting the rows
    /// of the grid. All rows of the list must have the same length or it will
    /// panic.
    ///
    /// Note that this is mostly used to create quick grids on the fly for
    /// testing. In real life situation, you will use the other constructors
    /// which offer better performance and flexibility.
    ///
    /// # Arguments
    ///
    /// * `rows` - A list of vectors with elements of each row
    ///
    /// # Panics
    ///
    /// This function panics if all vectors don't have the same length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Coordinate, Size, Grid, coord, size};
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    ///
    /// assert_eq!(grid.value(coord!(0, 0)), &1);
    /// assert_eq!(grid.value(coord!(1, 0)), &2);
    /// assert_eq!(grid.value(coord!(0, 1)), &3);
    /// assert_eq!(grid.value(coord!(1, 1)), &4);
    /// ```
    ///
    pub fn from_rows(rows: Vec<Vec<T>>) -> Grid<T> {
        // Todo: This implementation is naive and doesn't ensure the actual grid
        // capacity is correct; the rows should be manually recreated instead.
        let width: usize = rows.first().unwrap().len();
        let height: usize = rows.len();

        assert_eq!(rows.iter().all(|row| row.len() == width), true, "vectors don't have the same length");

        Grid::<T> {
            size: size!(width, height),
            rows: rows,
            row_capacity: width
        }
    }

    /// Create a grid from columns
    ///
    /// This function creates a grid from a list of vectors denoting the columns
    /// of the grid. All columns of the list must have the same length or it
    /// will panic.
    ///
    /// Note that this is mostly used to create quick grids on the fly for
    /// testing. In real life situation, you will use the other constructors
    /// which offer better performance and flexibility.
    ///
    /// # Arguments
    ///
    /// * `columns` - A list of vectors with elements of each columns
    ///
    /// # Panics
    ///
    /// This function panics if all vectors don't have the same length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Coordinate, Size, Grid, coord, size};
    /// #
    /// let grid = Grid::from_columns(vec![vec![1, 3],
    ///                                    vec![2, 4]]);
    ///
    /// assert_eq!(grid.value(coord!(0, 0)), &1);
    /// assert_eq!(grid.value(coord!(1, 0)), &2);
    /// assert_eq!(grid.value(coord!(0, 1)), &3);
    /// assert_eq!(grid.value(coord!(1, 1)), &4);
    /// ```
    ///
    pub fn from_columns(columns: Vec<Vec<T>>) -> Grid<T> {
        // Perhaps not the most efficient implementation.
        let mut grid = Self::from_rows(columns);

        grid.flip_horizontally();
        grid.rotate_left();

        grid
    }

    /// Create an empty grid.
    ///
    /// This method is equivalent to the `new()` constructor. Use it to make
    /// your code more readable.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let grid = Grid::<()>::zero();
    /// assert_eq!(grid.size(), size!(0, 0));
    /// ```
    ///
    pub fn zero() -> Grid<T> {
        Self::new()
    }

    /// Return the size of the grid.
    ///
    /// This method returns the size of the grid. Indirectly, that allows one to
    /// compute the actual number of elements in the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// assert_eq!(grid.size(), size!(2, 2));
    /// grid.resize(size!(5, 5), 42);
    /// assert_eq!(grid.size(), size!(5, 5));
    /// ```
    ///
    pub fn size(&self) -> Size {
        self.size
    }

    /// Resize the grid
    ///
    /// This method resizes the grid, adding more elements to it and/or dropping
    /// existing values. It resizes it with a given value which is cloned when
    /// the grid grows on one of the two axis.
    ///
    /// Note that it increases the size of the grid and if the capacity isn't
    /// high enough, reallocation occurs.
    ///
    /// # Arguments
    ///
    /// * `size`   - The new size of the grid
    /// * `value`  - The value to be cloned
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Coordinate, Size, Grid, coord, size};
    /// #
    /// let mut grid = Grid::zero();
    /// grid.resize(size!(2, 2), 42);
    ///
    /// assert_eq!(grid.size(), size!(2, 2));
    /// assert_eq!(grid[coord!(0, 0)], 42);
    /// assert_eq!(grid[coord!(1, 0)], 42);
    /// assert_eq!(grid[coord!(0, 1)], 42);
    /// assert_eq!(grid[coord!(1, 1)], 42);
    /// ```
    ///
    pub fn resize(&mut self, size: Size, value: T) {
        let row_capacity = if self.row_capacity < size.width {
            size.width
        } else {
            self.row_capacity
        };

        if size.height > self.rows.len() {

            self.rows.resize_with(size.height, || {
                let mut row = Vec::<T>::with_capacity(row_capacity);
                row.resize(size.width, value.clone());

                row
            });
        }

        for row in 0..size.height {
            self.rows[row].resize(size.width, value.clone());
        }

        for row in size.height..self.rows.len() {
            self.rows[row].truncate(0);
        }

        self.size = size;
        self.row_capacity = row_capacity;
    }

    /// Fill the grid with a given value.
    ///
    /// This method fills the grid with a given value that is cloned for all
    /// the elements.
    ///
    /// # Arguments
    ///
    /// * `value` - Value to fill the the grid with.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// grid.fill(42);
    /// assert!(grid.iterator().all(|item| *item == 42))
    /// ```
    ///
    pub fn fill(&mut self, value: T) {
        for i in 0..self.size.height {
            for item in self.rows[i].iter_mut() {
                *item = value.clone();
            }
        }
    }

    /// Clear the grid by removing all values.
    ///
    /// This method clears the grid by removing all values and therefore setting
    /// its size to zero.
    ///
    /// Note that this method has no effect on the allocated capacity of the
    /// grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// grid.clear();
    /// assert_eq!(grid.size(), size!(0, 0));
    /// assert_eq!(grid.capacity(), size!(2, 2));
    /// ```
    ///
    pub fn clear(&mut self) {
        for row in self.rows.iter_mut() {
            row.clear();
        }

        self.size = size!(0, 0);
    }

    /// Return a reference to an element of the grid.
    ///
    /// This method returns a reference to an element of the grid from its
    /// coordinate.
    ///
    /// Note that coordinate (0, 0) corresponds to the top-left element in the
    /// grid.
    ///
    /// # Arguments
    ///
    /// * `coordinate` - Coordinate of the element
    ///
    /// # Panics
    ///
    /// It panics if the coordinate is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// # use ingrid::{Coordinate, Grid, coord};
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// assert_eq!(grid.value(coord!(0, 0)), &1);
    /// assert_eq!(grid.value(coord!(1, 1)), &4);
    ///
    /// grid.value(coord!(2, 0)); // It panics here !
    /// ```
    ///
    pub fn value(&self, coordinate: Coordinate) -> &T {
        assert!(coordinate.x < self.size.width, "index out of bounds");
        assert!(coordinate.y < self.size.height, "index out of bounds");

        &self.rows[coordinate.y][coordinate.x]
    }

    /// Return a mutable reference to an element of the grid.
    ///
    /// This method returns a mutable reference to an element of the grid from
    /// its coordinate.
    ///
    /// # Panics
    ///
    /// It panics if the coordinate is out of bounds.
    ///
    /// # Arguments
    ///
    /// * `coordinate` - Coordinate of the element
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// # use ingrid::{Coordinate, Grid, coord};
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 0]]);
    ///
    /// let value = grid.value_mut(coord!(1, 1));
    /// *value = 4;
    ///
    /// assert_eq!(grid.value_mut(coord!(0, 0)), &1);
    /// assert_eq!(grid.value_mut(coord!(1, 1)), &4);
    ///
    /// grid.value(coord!(2, 0)); // It panics here !
    /// ```
    ///
    pub fn value_mut<'a>(&'a mut self, coordinate: Coordinate) -> &'a mut T {
        assert!(coordinate.x < self.size.width, "index out of bounds");
        assert!(coordinate.y < self.size.height, "index out of bounds");

        self.rows.get_mut(coordinate.y).unwrap().get_mut(coordinate.x).unwrap()
    }

    /// Replace an element of the grid.
    ///
    /// This method replaces the value of an element of the grid from its
    /// coordinate and a new value, effectively dropping the previous value.
    ///
    /// # Arguments
    ///
    /// * `coordinate` - Coordinate of the element
    /// * `value` - New value of the element
    ///
    /// # Panics
    ///
    /// It panics if the coordinate is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// # use ingrid::{Coordinate, Grid, coord};
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 0]]);
    ///
    /// grid.set_value(coord!(1, 1), 4);
    ///
    /// assert_eq!(grid.value_mut(coord!(0, 0)), &1);
    /// assert_eq!(grid.value_mut(coord!(1, 1)), &4);
    ///
    /// grid.set_value(coord!(2, 0), 5); // It panics here !
    /// ```
    ///
    pub fn set_value(&mut self, coordinate: Coordinate, value: T) {
        assert!(coordinate.x < self.size.width, "index out of bounds");
        assert!(coordinate.y < self.size.height, "index out of bounds");

        self.rows[coordinate.y][coordinate.x] = value;
    }

    /// Swap two elements of the grid.
    ///
    /// This method swaps two elements of the grid from their coordinates.
    ///
    /// # Arguments
    ///
    /// * `a` - Coordinate of one of the element to swap
    /// * `b` - Coordinate of the other element to be swapped with
    ///
    /// # Panics
    ///
    /// It panics if the coordinates are out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// # use ingrid::{Coordinate, Grid, coord};
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![4, 2],
    ///                                     vec![3, 1]]);
    ///
    /// grid.swap_value(coord!(0, 0), coord!(1, 1));
    ///
    /// assert_eq!(grid.value(coord!(0, 0)), &1);
    /// assert_eq!(grid.value(coord!(1, 1)), &4);
    ///
    /// grid.swap_value(coord!(2, 0), coord!(0, 0)); // It panics here !
    /// ```
    ///
    pub fn swap_value(&mut self, a: Coordinate, b: Coordinate) {
        assert!(a.x < self.size.width, "index out of bounds");
        assert!(a.y < self.size.height, "index out of bounds");

        assert!(b.x < self.size.width, "index out of bounds");
        assert!(b.y < self.size.height, "index out of bounds");

        // checkout: https://stackoverflow.com/questions/30073684/how-to-get-mutable-references-to-two-array-elements-at-the-same-time
        unsafe {
            let foo = &mut *(self.rows.get_mut(a.y).unwrap().get_unchecked_mut(a.x) as *mut _);
            let bar = &mut *(self.rows.get_mut(b.y).unwrap().get_unchecked_mut(b.x) as *mut _);

            std::mem::swap(foo, bar);
        }
    }

    /// Return the elements of the grid.
    ///
    /// This method returns the elements of the grid as a vector of reference.
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    ///
    /// assert_eq!(grid.values(), vec![&1, &2, &3, &4]);
    /// ```
    ///
    pub fn values(&self) -> Vec<&T> {
        self.iterator().collect()
    }

    /// Returns an iterator over the grid.
    ///
    /// This method returns an iterator over the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// let mut iterator = grid.iterator();
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&3));
    /// assert_eq!(iterator.next(), Some(&4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    ///
    pub fn iterator<'a>(&'a self) -> IteratorGrid<'a, T> {
        IteratorGrid::new(self)
    }

    /// Create a view onto a given row
    ///
    /// This method creates a view onto a given row of the grid. The row is
    /// immutable; use `row_mut()` to compute a mutable row.
    ///
    /// # Panics
    ///
    /// It panics if the index is out of bounds (less than the height of the
    /// grid).
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the row
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    ///
    /// assert_eq!(grid.row(1).values(), vec![&3, &4]);
    /// ```
    ///
    pub fn row<'a>(&'a self, index: usize) -> Row<'a, T> {
        assert!(index < self.size.height, "index out of bounds");

        Row {
            grid: self,
            index: index
        }
    }

    /// Create a view onto a given row
    ///
    /// This method creates a view onto a given row of the grid. The row is
    /// mutable; use `row()` to compute an immutable row.
    ///
    /// # Panics
    ///
    /// It panics if the index is out of bounds (less than the height of the
    /// grid).
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the row
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![0, 0]]);
    ///
    /// let mut row = grid.row_mut(1);
    /// row[0] = 3;
    /// row[1] = 4;
    ///
    /// assert_eq!(grid.row(1).values(), vec![&3, &4]);
    /// ```
    ///
    pub fn row_mut<'a>(&'a mut self, index: usize) -> RowMut<'a, T> {
        assert!(index < self.size.height, "index out of bounds");

        RowMut {
            grid: self,
            index: index
        }
    }

    /// Swap two rows of the grid.
    ///
    /// This method swaps two rows of the grid from their index.
    ///
    /// # Arguments
    ///
    /// * `a` - Index of one of the row to swap
    /// * `b` - Index of the other row to be swapped with
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
    ///                                     vec![4, 5, 6]]);
    ///
    /// grid.swap_row(0, 1);
    ///
    /// assert_eq!(grid.row(0).values(), vec![&4, &5, &6]);
    /// assert_eq!(grid.row(1).values(), vec![&1, &2, &3]);
    ///
    /// grid.swap_row(1, 2); // It panics here !
    /// ```
    ///
    pub fn swap_row(&mut self, a: usize, b: usize) {
        assert!(a < self.size.height, "index out of bounds");
        assert!(b < self.size.height, "index out of bounds");

        self.rows.swap(a, b);
    }

    /// Return the rows of the grid
    ///
    /// This method returns the rows of the grid as a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    ///
    /// let rows = grid.rows();
    /// assert_eq!(rows[0].values(), vec![&1, &2]);
    /// assert_eq!(rows[1].values(), vec![&3, &4]);
    /// ```
    ///
    pub fn rows<'a>(&'a self) -> Vec<Row<'a, T>> {
        let mut rows = Vec::with_capacity(self.size.height);

        for index in 0..self.size.height {
            rows.push(self.row(index));
        }

        rows
    }

    /// Insert a row into the grid
    ///
    /// This method inserts a row into the grid at position `index`, shifting
    /// all rows after it to the bottom. The row is a vector holding the
    /// elements of the inserted row, which are then moved to the grid. Its
    /// length must be equal to the length as the other rows.
    ///
    /// Note that it increases the size of the grid and if the capacity isn't
    /// high enough, reallocation occurs.
    ///
    /// # Arguments
    ///
    /// * `index` - Position index of the inserted row
    /// * `row` - Vector with the element of the new row
    ///
    /// # Panics
    ///
    /// It panics if the index is out of bounds or if the length of the vector
    /// doesn't equal the length of the other rows.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![7, 8, 9]]);
    ///
    /// grid.insert_row(1, vec![4, 5, 6]);
    ///
    /// assert_eq!(grid.column(0).values(), vec![&1, &4, &7]);
    /// assert_eq!(grid.column(1).values(), vec![&2, &5, &8]);
    /// assert_eq!(grid.column(2).values(), vec![&3, &6, &9]);
    /// ```
    ///
    pub fn insert_row(&mut self, index: usize, row: Vec<T>) {
        assert!(!(index > self.size.height), "index out of bounds"); // syntax -- wtf!!
        assert_eq!(row.len(), self.size.width, "row length is invalid");

        // The capacity doesn't change unless it's too small
        if self.size.height < self.rows.len() {
            self.rows.pop();
            self.rows.insert(index, row);
        }
        else {
            self.rows.insert(index, row);
        }

        self.size.height += 1;
    }

    /// Remove a row from the grid.
    ///
    /// This method removes a row from the grid at position index, shifting all
    /// rows after it to the top.
    ///
    /// Note that this method has no effect on the allocated capacity of the
    /// grid.
    ///
    /// # Arguments
    ///
    /// * `index` - Position index of the row to remove
    ///
    /// # Panics
    ///
    /// It panics if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![4, 5, 6],
    ///                                     vec![7, 8, 9]]);
    ///
    /// grid.remove_row(1);
    ///
    /// assert_eq!(grid.column(0).values(), vec![&1, &7]);
    /// assert_eq!(grid.column(1).values(), vec![&2, &8]);
    /// assert_eq!(grid.column(2).values(), vec![&3, &9]);
    /// ```
    ///
    pub fn remove_row(&mut self, index: usize) {
        assert!(index < self.size.height, "index out of bounds");

        // Removing a row doesn't change the capacity of the grid.
        self.rows.remove(index);
        self.rows.push(Vec::<T>::with_capacity(self.row_capacity));

        self.size.height -= 1;
    }

    /// Create a view onto a given column
    ///
    /// This method creates a view onto a given column of the grid. The column
    /// is immutable; use `column_mut()` to compute a mutable column.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out of bounds (less than the
    /// width of the grid).
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the column
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    ///
    /// assert_eq!(grid.column(1).values(), vec![&2, &4]);
    /// ```
    ///
    pub fn column<'a>(&'a self, index: usize) -> Column<'a, T> {
        assert!(index < self.size.width, "index out of bounds");

        Column {
            grid: self,
            index: index
        }
    }

    /// Create a view onto a given column
    ///
    /// This method creates a view onto a given column of the grid. The column
    /// is mutable; use `column()` to compute a immutable column.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out of bounds (less than the
    /// width of the grid).
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the column
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 0],
    ///                                     vec![3, 0]]);
    ///
    /// let mut column = grid.column_mut(1);
    /// column[0] = 2;
    /// column[1] = 4;
    ///
    /// assert_eq!(grid.column(1).values(), vec![&2, &4]);
    /// ```
    ///
    pub fn column_mut<'a>(&'a mut self, index: usize) -> ColumnMut<'a, T> {
        assert!(index < self.size.width, "index out of bounds");

        ColumnMut {
            grid: self,
            index: index
        }
    }

    /// Swap two columns of the grid.
    ///
    /// This method swaps two columns of the grid from their index.
    ///
    /// # Arguments
    ///
    /// * `a` - Index of one of the column to swap
    /// * `b` - Index of the other column to be swapped with
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
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4],
    ///                                     vec![5, 6]]);
    ///
    /// grid.swap_column(0, 1);
    ///
    /// assert_eq!(grid.column(0).values(), vec![&2, &4, &6]);
    /// assert_eq!(grid.column(1).values(), vec![&1, &3, &5]);
    ///
    /// grid.swap_column(1, 2); // It panics here !
    /// ```
    ///
    pub fn swap_column(&mut self, a: usize, b: usize) {
        assert!(a < self.size.width, "index out of bounds");
        assert!(b < self.size.width, "index out of bounds");

        for index in 0..self.size.height {
            self.rows[index].swap(a, b);
        }
    }

    /// Return the columns of the grid
    ///
    /// This method returns the columns of the grid as a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let grid = Grid::from_rows(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    ///
    /// let columns = grid.columns();
    /// assert_eq!(columns[0].values(), vec![&1, &3]);
    /// assert_eq!(columns[1].values(), vec![&2, &4]);
    /// ```
    ///
    pub fn columns<'a>(&'a self) -> Vec<Column<'a, T>> {
        let mut columns = Vec::with_capacity(self.size.width);

        for index in 0..self.size.width {
            columns.push(self.column(index));
        }

        columns
    }

    /// Insert a column into the grid
    ///
    /// This method inserts a column into the grid at position `index`, shifting
    /// all columns after it to the right. The column is a vector holding the
    /// elements of the inserted column, which are then moved to the grid. Its
    /// length must be equal to the length as the other columns.
    ///
    /// Note that it increases the size of the grid and if the capacity isn't
    /// high enough, reallocation occurs.
    ///
    /// # Arguments
    ///
    /// * `index` - Position index of the inserted column
    /// * `column` - Vector with the element of the new column
    ///
    /// # Panics
    ///
    /// It panics if the index is out of bounds or if the length of the vector
    /// doesn't equal the length of the other columns.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 3],
    ///                                     vec![4, 6],
    ///                                     vec![7, 9]]);
    ///
    /// grid.insert_column(1, vec![2, 5, 8]);
    ///
    /// assert_eq!(grid.row(0).values(), vec![&1, &2, &3]);
    /// assert_eq!(grid.row(1).values(), vec![&4, &5, &6]);
    /// assert_eq!(grid.row(2).values(), vec![&7, &8, &9]);
    /// ```
    ///
    pub fn insert_column(&mut self, index: usize, mut column: Vec<T>) {
        assert!(!(index > self.size.width), "index out of bounds");
        assert_eq!(column.len(), self.size.height, "column length is invalid");

        // The capacity doesn't change unless it's too small
        if self.size.width + 1 > self.row_capacity {
            self.row_capacity += 1;
        }

        for i in 0..self.size.height {
            self.rows[i].insert(index, column.remove(0));
        }
        assert_eq!(column.len(), 0);


        self.size.width += 1;
    }

    /// Remove a column from the grid.
    ///
    /// This method removes a column from the grid at position index, shifting
    /// all columns after it to the left.
    ///
    /// Note that this method has no effect on the allocated capacity of the
    /// grid.
    ///
    /// # Arguments
    ///
    /// * `index` - Position index of the column to remove
    ///
    /// # Panics
    ///
    /// It panics if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Grid;
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
    ///                                     vec![4, 5, 6],
    ///                                     vec![7, 8, 9]]);
    ///
    /// grid.remove_column(1);
    ///
    /// assert_eq!(grid.row(0).values(), vec![&1, &3]);
    /// assert_eq!(grid.row(1).values(), vec![&4, &6]);
    /// assert_eq!(grid.row(2).values(), vec![&7, &9]);
    /// ```
    ///
    pub fn remove_column(&mut self, index: usize) {
        assert!(index < self.size.width, "index out of bounds");

        // Removing a column doesn't change the capacity of the grid.
        for row in 0..self.size.height {
            self.rows[row].remove(index);
        }

        self.size.width -= 1;
    }

    /// Flip the grid horizontally
    ///
    /// This method flips the grid horizontally, reversing the order of the
    /// elements of each row, one by one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// grid.flip_horizontally();
    /// assert_eq!(grid.row(0).values(), vec![&2, &1]);
    /// assert_eq!(grid.row(1).values(), vec![&4, &3]);
    /// ```
    ///
    pub fn flip_horizontally(&mut self) {
        for index in 0..self.size.height {
            self.row_mut(index).reverse();
        }
    }

    /// Flip the grid vertically
    ///
    /// This method flips the grid vertically, reversing the order of the
    /// elements of each column, one by one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// grid.flip_vertically();
    /// assert_eq!(grid.row(0).values(), vec![&3, &4]);
    /// assert_eq!(grid.row(1).values(), vec![&1, &2]);
    /// ```
    ///
    pub fn flip_vertically(&mut self) {
        for index in 0..self.size.width {
            self.column_mut(index).reverse();
        }
    }

    /// Rotate the grid to the left
    ///
    /// This method rotate the grid to the left, rearranging its elements.
    ///
    /// Note that the capacity of the grid is also rotated; if capacity was
    /// (a, b), this is now (b, a).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// grid.rotate_left();
    /// assert_eq!(grid.row(0).values(), vec![&2, &4]);
    /// assert_eq!(grid.row(1).values(), vec![&1, &3]);
    /// ```
    ///
    pub fn rotate_left(&mut self) {
        // Rotation cannot be done in-place, therefore, the strategy is to
        // create another grid, then swap them
        let size = size!(self.size.height, self.size.width);
        let mut grid = Self::with_capacity(size);

        for i in 0..self.size.height {
            for j in 0..self.size.width {
                grid.rows[j].push(self.rows[i].pop().unwrap());
            }
        }

        grid.size = size;

        std::mem::swap(self, &mut grid);
    }

    /// Rotate the grid to the right
    ///
    /// This method rotate the grid to the right, rearranging its elements.
    ///
    /// Note that the capacity of the grid is also rotated; if capacity was
    /// (a, b), this is now (b, a).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let mut grid = Grid::from_rows(vec![vec![1, 2],
    ///                                     vec![3, 4]]);
    ///
    /// grid.rotate_right();
    /// assert_eq!(grid.row(0).values(), vec![&3, &1]);
    /// assert_eq!(grid.row(1).values(), vec![&4, &2]);
    /// ```
    ///
    pub fn rotate_right(&mut self) {
        // Rotation cannot be done in-place, therefore, the strategy is to
        // create another grid, then swap them
        let size = size!(self.size.height, self.size.width);
        let mut grid = Self::with_capacity(size);

        for i in (0..self.size.height).rev() {
            for j in (0..self.size.width).rev() {
                grid.rows[j].push(self.rows[i].pop().unwrap());
            }
        }

        grid.size = size;

        std::mem::swap(self, &mut grid);
    }

    /// Return the number of elements the grid can hold without reallocating.
    ///
    /// This method returns the number of elements the grid can hold without
    /// reallocating on both axis.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let grid = Grid::<()>::with_capacity(size!(2, 3));
    /// assert_eq!(grid.capacity(), size!(2, 3));
    /// ```
    ///
    pub fn capacity(&self) -> Size {
        size!(self.row_capacity, self.rows.len())
    }

    /// Reserve capacity for at least additional more elements to be inserted
    ///
    /// This method reserves capacity for at least additional more elements to
    /// be inserted in the grid. The collection may reserve more space to avoid
    /// frequent reallocations. After calling reserve, capacity will be greater
    /// than or equal to `self.size() + additional`. Does nothing if capacity is
    /// already sufficient.
    ///
    /// # Arguments
    ///
    /// * `additional` - Capacity to be added on both axis
    ///
    /// # Panics
    ///
    /// It panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::{Size, Grid, size};
    /// #
    /// let mut grid = Grid::<()>::with_capacity(size!(2, 3));
    /// grid.reserve(size!(3, 2));
    /// assert_eq!(grid.capacity(), size!(5, 5));
    /// ```
    ///
    pub fn reserve(&mut self, additional: Size) {
        for i in 0..self.size.height {
            self.rows[i].reserve_exact(additional.width);
        }

        self.row_capacity += additional.width;

        self.rows.reserve_exact(additional.height);
        let foobar = self.rows.capacity().clone();

        let row_capacity = self.row_capacity;
        self.rows.resize_with(foobar, || Vec::<T>::with_capacity(row_capacity));
    }

    // unfinished
    pub fn row_slice(&mut self, row: usize) -> &mut [T] {
        assert!(row < self.size.height, "index out of bounds");
        self.rows[row].as_mut_slice()
    }
}

impl<T> Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, coordinate: Coordinate) -> &Self::Output {
        &self.rows[coordinate.y][coordinate.x]
    }
}

impl<T> IndexMut<Coordinate> for Grid<T> {
    fn index_mut(&mut self, coordinate: Coordinate) -> &mut Self::Output {
        &mut self.rows[coordinate.y][coordinate.x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_new() {
        let grid = Grid::<()>::new();
        assert_eq!(grid.size(), size!(0, 0));
        assert_eq!(grid.capacity(), size!(0, 0));
    }

    #[test]
    fn grid_with_size() {
        let grid = Grid::with_size(size!(2, 3), 42);

        assert_eq!(grid.size(), size!(2, 3));
        assert!(grid.iterator().all(|item| { *item == 42 }), true);

        assert_eq!(grid.capacity(), size!(2, 3));
    }

    #[test]
    fn grid_with_capacity() {
        let grid = Grid::<()>::with_capacity(size!(5, 5));

        assert_eq!(grid.size(), size!(0, 0));
        assert_eq!(grid.capacity(), size!(5, 5));
    }

    #[test]
    fn grid_size() {
        let mut grid = Grid::zero();
        assert_eq!(grid.size(), size!(0, 0));

        grid.resize(size!(3, 0), 42);
        assert_eq!(grid.size(), size!(3, 0));

        grid.resize(size!(0, 3), 42);
        assert_eq!(grid.size(), size!(0, 3));

        grid.resize(size!(3, 3), 42);
        assert_eq!(grid.size(), size!(3, 3));
    }

    #[test]
    fn grid_resize() {
        // [0,  0, 0] => [ 0]
        // [0, 42, 0]    [ 0]
        // [0,  0, 0]    [ 0]
        //               [42]
        //               [42]
        let mut grid = Grid::from_rows(vec![vec![0,  0, 0],
                                            vec![0, 42, 0],
                                            vec![0,  0, 0]]);

        grid.resize(size!(1, 5), 42);
        assert_eq!(grid.size(), size!(1, 5));

        assert_eq!(grid.value(coord!(0, 0)), &0);
        assert_eq!(grid.value(coord!(0, 1)), &0);
        assert_eq!(grid.value(coord!(0, 2)), &0);
        assert_eq!(grid.value(coord!(0, 3)), &42);
        assert_eq!(grid.value(coord!(0, 4)), &42);

        // Capacity doesn't change unless it's too small.
        assert_eq!(grid.capacity(), size!(3, 5));
    }

    #[test]
    fn grid_fill() {
        let mut grid = Grid::with_size(size!(3, 3), 0);
        assert_eq!(grid.iterator().all(|item| { *item == 42 }), false);

        grid.fill(42);
        assert_eq!(grid.iterator().all(|item| { *item == 42 }), true);
    }

    #[test]
    fn grid_clear() {
        let mut grid = Grid::zero();

        grid.clear();
        assert_eq!(grid.size(), size!(0, 0));
        assert_eq!(grid.capacity(), size!(0, 0));

        grid.resize(size!(3, 0), 42);
        grid.clear();
        assert_eq!(grid.size(), size!(0, 0));
        assert_eq!(grid.capacity(), size!(3, 0));

        grid.resize(size!(0, 3), 42);
        grid.clear();
        assert_eq!(grid.size(), size!(0, 0));
        assert_eq!(grid.capacity(), size!(3, 3));

        grid.resize(size!(5, 5), 42);
        grid.clear();
        assert_eq!(grid.size(), size!(0, 0));
        assert_eq!(grid.capacity(), size!(5, 5));
    }

    #[test]
    #[should_panic(expected = "vectors don't have the same length")]
    fn grid_from_rows() {
        let grid = Grid::from_rows(vec![vec![1, 2, 3], vec![4, 5, 6]]);

        assert_eq!(grid.size(), size!(3, 2));
        assert_eq!(grid.value(coord!(0, 0)), &1);
        assert_eq!(grid.value(coord!(1, 0)), &2);
        assert_eq!(grid.value(coord!(2, 0)), &3);
        assert_eq!(grid.value(coord!(0, 1)), &4);
        assert_eq!(grid.value(coord!(1, 1)), &5);
        assert_eq!(grid.value(coord!(2, 1)), &6);

        assert_eq!(grid.capacity(), size!(3, 2));

        Grid::from_rows(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]]);
    }

    #[test]
    #[should_panic(expected = "vectors don't have the same length")]
    fn grid_from_columns() {
        let grid = Grid::from_columns(vec![vec![1, 3, 5], vec![2, 4, 6]]);

        assert_eq!(grid.size(), size!(2, 3));
        assert_eq!(grid.value(coord!(0, 0)), &1);
        assert_eq!(grid.value(coord!(1, 0)), &2);
        assert_eq!(grid.value(coord!(0, 1)), &3);
        assert_eq!(grid.value(coord!(1, 1)), &4);
        assert_eq!(grid.value(coord!(0, 2)), &5);
        assert_eq!(grid.value(coord!(1, 2)), &6);

        assert_eq!(grid.capacity(), size!(2, 3));

        Grid::from_columns(vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6]]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_value() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        assert_eq!(grid.value(coord!(0, 0)), &1);
        assert_eq!(grid.value(coord!(1, 0)), &2);
        assert_eq!(grid.value(coord!(0, 1)), &3);
        assert_eq!(grid.value(coord!(1, 1)), &4);

        grid.value(coord!(0, 2));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_value_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 0]]);

        *grid.value_mut(coord!(1, 1)) = 4;

        assert_eq!(grid.value(coord!(0, 0)), &1);
        assert_eq!(grid.value(coord!(1, 0)), &2);
        assert_eq!(grid.value(coord!(0, 1)), &3);
        assert_eq!(grid.value(coord!(1, 1)), &4);

        grid.value_mut(coord!(0, 2));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_set_value() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 0]]);

        grid.set_value(coord!(1, 1), 4);

        assert_eq!(grid.value(coord!(0, 0)), &1);
        assert_eq!(grid.value(coord!(1, 0)), &2);
        assert_eq!(grid.value(coord!(0, 1)), &3);
        assert_eq!(grid.value(coord!(1, 1)), &4);

        grid.set_value(coord!(0, 2), 5);
    }

    #[test]
    fn grid_swap_value() {
        let mut grid = Grid::from_rows(vec![vec![1, 4],
                                            vec![3, 2]]);

        grid.swap_value(coord!(1, 0), coord!(1, 1));

        assert_eq!(grid.value(coord!(0, 0)), &1);
        assert_eq!(grid.value(coord!(1, 0)), &2);
        assert_eq!(grid.value(coord!(0, 1)), &3);
        assert_eq!(grid.value(coord!(1, 1)), &4);
    }

    #[test]
    fn grid_values() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        assert_eq!(grid.values(), vec![&1, &2, &3, &4]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_index() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        assert_eq!(grid[coord!(0, 0)], 1);
        assert_eq!(grid[coord!(1, 0)], 2);
        assert_eq!(grid[coord!(0, 1)], 3);
        assert_eq!(grid[coord!(1, 1)], 4);

        grid[coord!(0, 2)];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_index_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 0]]);

        grid[coord!(1, 1)] = 4;

        assert_eq!(grid[coord!(0, 0)], 1);
        assert_eq!(grid[coord!(1, 0)], 2);
        assert_eq!(grid[coord!(0, 1)], 3);
        assert_eq!(grid[coord!(1, 1)], 4);

        grid[coord!(0, 2)];
    }

    #[test]
    fn grid_iterator() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        let mut iterator = grid.iterator();
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_row() {
        let grid = Grid::from_rows(vec![vec![1, 2], vec![3, 4]]);

        assert_eq!(grid.row(0).values(), vec![&1, &2]);
        assert_eq!(grid.row(1).values(), vec![&3, &4]);

        grid.row(2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_row_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4]]);

        assert_eq!(grid.row_mut(0).values(), vec![&1, &2]);
        assert_eq!(grid.row_mut(1).values(), vec![&3, &4]);

        grid.row_mut(2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_swap_row() {
        let mut grid = Grid::from_rows(vec![vec![3, 4],
                                            vec![1, 2]]);

        grid.swap_row(0, 1);
        assert_eq!(grid.row(0).values(), vec![&1, &2]);
        assert_eq!(grid.row(1).values(), vec![&3, &4]);

        grid.swap_row(1, 2);
    }

    #[test]
    fn grid_rows() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        assert_eq!(grid.rows(), vec![grid.row(0), grid.row(1)]);
    }

    #[test]
    fn grid_insert_row() {
        let mut grid = Grid::from_rows(vec![vec![4, 5, 6]]);

        assert_eq!(grid.capacity(), size!(3, 1));

        // Test inserting a row at the very beginning.
        grid.insert_row(0, vec![1, 2, 3]);

        assert_eq!(grid.size(), size!(3, 2));
        assert_eq!(grid[coord!(0, 0)], 1);
        assert_eq!(grid[coord!(1, 0)], 2);
        assert_eq!(grid[coord!(2, 0)], 3);

        assert_eq!(grid.capacity(), size!(3, 2));

        // Test inserting a row at the very end
        grid.insert_row(2, vec![7, 8, 9]);

        assert_eq!(grid.size(), size!(3, 3));
        assert_eq!(grid[coord!(0, 2)], 7);
        assert_eq!(grid[coord!(1, 2)], 8);
        assert_eq!(grid[coord!(2, 2)], 9);

        assert_eq!(grid.capacity(), size!(3, 3));

        assert_eq!(grid[coord!(0, 1)], 4);
        assert_eq!(grid[coord!(1, 1)], 5);
        assert_eq!(grid[coord!(2, 1)], 6);
    }

    #[test]
    fn grid_insert_row_with_capacity() {
        let mut grid = Grid::from_rows(vec![vec![4, 5, 6]]);
        grid.reserve(size!(0, 2));

        assert_eq!(grid.capacity(), size!(3, 3));

        // Test inserting a row at the very beginning.
        grid.insert_row(0, vec![1, 2, 3]);

        assert_eq!(grid.size(), size!(3, 2));
        assert_eq!(grid[coord!(0, 0)], 1);
        assert_eq!(grid[coord!(1, 0)], 2);
        assert_eq!(grid[coord!(2, 0)], 3);

        assert_eq!(grid.capacity(), size!(3, 3));

        // Test inserting a row at the very end
        grid.insert_row(2, vec![7, 8, 9]);

        assert_eq!(grid.size(), size!(3, 3));
        assert_eq!(grid[coord!(0, 2)], 7);
        assert_eq!(grid[coord!(1, 2)], 8);
        assert_eq!(grid[coord!(2, 2)], 9);

        assert_eq!(grid.capacity(), size!(3, 3));

        assert_eq!(grid[coord!(0, 1)], 4);
        assert_eq!(grid[coord!(1, 1)], 5);
        assert_eq!(grid[coord!(2, 1)], 6);
    }

    #[test]
    fn grid_insert_row_zero() {
        // Test inserting a row in empty grid.
        let mut grid = Grid::<()>::new();
        grid.insert_row(0, vec![]);
        assert_eq!(grid.size(), size!(0, 1));

        let mut grid = Grid::new();
        grid.resize(size!(3, 0), 42);
        grid.insert_row(0, vec![1, 2, 3]);

        assert_eq!(grid[coord!(0, 0)], 1);
        assert_eq!(grid[coord!(1, 0)], 2);
        assert_eq!(grid[coord!(2, 0)], 3);

        assert_eq!(grid.size(), size!(3, 1));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_insert_row_invalid_index() {
        // Test inserting a row with invalid index.
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6]]);

        grid.insert_row(3, vec![7, 8, 9]);
    }

    #[test]
    #[should_panic(expected = "row length is invalid")]
    fn grid_insert_row_invalid_row() {
        // Test inserting a row with invalid elements.
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6]]);

        grid.insert_row(2, vec![7, 8]);
    }

    #[test]
    fn grid_remove_row() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        assert_eq!(grid.capacity(), size!(3, 3));

        // Test removing the row at the very beginning
        grid.remove_row(0);

        assert_eq!(grid.size(), size!(3, 2));
        assert_eq!(grid[coord!(0, 0)], 4);
        assert_eq!(grid[coord!(1, 0)], 5);
        assert_eq!(grid[coord!(2, 0)], 6);

        assert_eq!(grid.capacity(), size!(3, 3));

        // Test removing the row at the very end
        grid.remove_row(1);

        assert_eq!(grid.size(), size!(3, 1));
        assert_eq!(grid[coord!(0, 0)], 4);
        assert_eq!(grid[coord!(1, 0)], 5);
        assert_eq!(grid[coord!(2, 0)], 6);

        assert_eq!(grid.capacity(), size!(3, 3));

        // Test removing the very last row
        grid.remove_row(0);
        assert_eq!(grid.size(), size!(3, 0));

        assert_eq!(grid.capacity(), size!(3, 3));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_column() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        assert_eq!(grid.column(0).values(), vec![&1, &3]);
        assert_eq!(grid.column(1).values(), vec![&2, &4]);

        grid.column(2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_column_mut() {
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4]]);

        assert_eq!(grid.column_mut(0).values(), vec![&1, &3]);
        assert_eq!(grid.column_mut(1).values(), vec![&2, &4]);

        grid.column_mut(2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_swap_column() {
        let mut grid = Grid::from_rows(vec![vec![2, 1],
                                            vec![4, 3]]);

        grid.swap_column(0, 1);
        assert_eq!(grid.column(0).values(), vec![&1, &3]);
        assert_eq!(grid.column(1).values(), vec![&2, &4]);

        grid.swap_column(1, 2);
    }

    #[test]
    fn grid_columns() {
        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);

        assert_eq!(grid.columns(), vec![grid.column(0), grid.column(1)]);
    }

    #[test]
    fn grid_insert_column() {
        let mut grid = Grid::from_rows(vec![vec![2],
                                            vec![5],
                                            vec![8]]);

        assert_eq!(grid.capacity(), size!(1, 3));

        // Test inserting a column the very beginning.
        grid.insert_column(0, vec![1, 4, 7]);

        assert_eq!(grid.size(), size!(2, 3));
        assert_eq!(grid[coord!(0, 0)], 1);
        assert_eq!(grid[coord!(0, 1)], 4);
        assert_eq!(grid[coord!(0, 2)], 7);

        assert_eq!(grid.capacity(), size!(2, 3));

        // Test inserting a column at the very end
        grid.insert_column(2, vec![3, 6, 9]);

        assert_eq!(grid.size(), size!(3, 3));
        assert_eq!(grid[coord!(2, 0)], 3);
        assert_eq!(grid[coord!(2, 1)], 6);
        assert_eq!(grid[coord!(2, 2)], 9);

        assert_eq!(grid.capacity(), size!(3, 3));

        assert_eq!(grid[coord!(1, 0)], 2);
        assert_eq!(grid[coord!(1, 1)], 5);
        assert_eq!(grid[coord!(1, 2)], 8);
    }

    #[test]
    fn grid_insert_column_with_capacity() {
        let mut grid = Grid::from_rows(vec![vec![2],
                                            vec![5],
                                            vec![8]]);
        grid.reserve(size!(2, 0));

        assert_eq!(grid.capacity(), size!(3, 3));

        // Test inserting a column the very beginning.
        grid.insert_column(0, vec![1, 4, 7]);

        assert_eq!(grid.size(), size!(2, 3));
        assert_eq!(grid[coord!(0, 0)], 1);
        assert_eq!(grid[coord!(0, 1)], 4);
        assert_eq!(grid[coord!(0, 2)], 7);

        assert_eq!(grid.capacity(), size!(3, 3));

        // Test inserting a column at the very end
        grid.insert_column(2, vec![3, 6, 9]);

        assert_eq!(grid.size(), size!(3, 3));
        assert_eq!(grid[coord!(2, 0)], 3);
        assert_eq!(grid[coord!(2, 1)], 6);
        assert_eq!(grid[coord!(2, 2)], 9);

        assert_eq!(grid.capacity(), size!(3, 3));

        assert_eq!(grid[coord!(1, 0)], 2);
        assert_eq!(grid[coord!(1, 1)], 5);
        assert_eq!(grid[coord!(1, 2)], 8);
    }

    #[test]
    fn grid_insert_column_zero() {
        // Test inserting a column in empty grid.
        let mut grid = Grid::<()>::new();
        grid.insert_column(0, vec![]);
        assert_eq!(grid.size(), size!(1, 0));

        let mut grid = Grid::new();
        grid.resize(size!(0, 3), 42);
        grid.insert_column(0, vec![1, 4, 7]);

        assert_eq!(grid[coord!(0, 0)], 1);
        assert_eq!(grid[coord!(0, 1)], 4);
        assert_eq!(grid[coord!(0, 2)], 7);

        assert_eq!(grid.size(), size!(1, 3));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn grid_insert_column_invalid_index() {
        // Test inserting a column with invalid index.
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![4, 5],
                                            vec![7, 8]]);

        grid.insert_column(3, vec![3, 6, 9]);
    }

    #[test]
    #[should_panic(expected = "column length is invalid")]
    fn grid_insert_column_invalid_column() {
        // Test inserting a column with invalid elements.
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![4, 5],
                                            vec![7, 8]]);

        grid.insert_column(2, vec![3, 6]);
    }

    #[test]
    fn grid_remove_column() {
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        assert_eq!(grid.capacity(), size!(3, 3));

        // Test removing the column at the very beginning
        grid.remove_column(0);

        assert_eq!(grid.size(), size!(2, 3));
        assert_eq!(grid[coord!(0, 0)], 2);
        assert_eq!(grid[coord!(0, 1)], 5);
        assert_eq!(grid[coord!(0, 2)], 8);

        assert_eq!(grid.capacity(), size!(3, 3));

        // Test removing the column at the very end
        grid.remove_column(1);

        assert_eq!(grid.size(), size!(1, 3));
        assert_eq!(grid[coord!(0, 0)], 2);
        assert_eq!(grid[coord!(0, 1)], 5);
        assert_eq!(grid[coord!(0, 2)], 8);

        assert_eq!(grid.capacity(), size!(3, 3));

        // Test removing the very last column
        grid.remove_column(0);
        assert_eq!(grid.size(), size!(0, 3));

        assert_eq!(grid.capacity(), size!(3, 3));
    }

    #[test]
    fn grid_flip_horizontally() {
        // [1, 2, 3] => [3, 2, 1]
        // [4, 5, 6]    [6, 5, 4]
        // [7, 8, 9]    [9, 8, 7]
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.flip_horizontally();

        assert_eq!(grid.size(), size!(3, 3));
        assert_eq!(grid.capacity(), size!(3, 3));

        assert_eq!(grid[coord!(0, 0)], 3);
        assert_eq!(grid[coord!(1, 0)], 2);
        assert_eq!(grid[coord!(2, 0)], 1);
        assert_eq!(grid[coord!(0, 1)], 6);
        assert_eq!(grid[coord!(1, 1)], 5);
        assert_eq!(grid[coord!(2, 1)], 4);
        assert_eq!(grid[coord!(0, 2)], 9);
        assert_eq!(grid[coord!(1, 2)], 8);
        assert_eq!(grid[coord!(2, 2)], 7);

        // [1, 2, 3] => [3, 2, 1]
        // [4, 5, 6]    [6, 5, 4]
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6]]);

        grid.flip_horizontally();

        assert_eq!(grid.size(), size!(3, 2));
        assert_eq!(grid.capacity(), size!(3, 2));

        assert_eq!(grid[coord!(0, 0)], 3);
        assert_eq!(grid[coord!(1, 0)], 2);
        assert_eq!(grid[coord!(2, 0)], 1);
        assert_eq!(grid[coord!(0, 1)], 6);
        assert_eq!(grid[coord!(1, 1)], 5);
        assert_eq!(grid[coord!(2, 1)], 4);

        // [1, 2] => [2, 1]
        // [3, 4]    [4, 3]
        // [5, 6]    [6, 5]
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4],
                                            vec![5, 6]]);

        grid.flip_horizontally();

        assert_eq!(grid.size(), size!(2, 3));
        assert_eq!(grid.capacity(), size!(2, 3));

        assert_eq!(grid[coord!(0, 0)], 2);
        assert_eq!(grid[coord!(1, 0)], 1);
        assert_eq!(grid[coord!(0, 1)], 4);
        assert_eq!(grid[coord!(1, 1)], 3);
        assert_eq!(grid[coord!(0, 2)], 6);
        assert_eq!(grid[coord!(1, 2)], 5);

        // [1] => [1]
        let mut grid = Grid::from_rows(vec![vec![1]]);

        grid.flip_horizontally();

        assert_eq!(grid.size(), size!(1, 1));
        assert_eq!(grid.capacity(), size!(1, 1));

        assert_eq!(grid[coord!(0, 0)], 1);

        // [] => []
        let mut grid = Grid::<()>::zero();

        grid.flip_horizontally();

        assert_eq!(grid.size(), size!(0, 0));
        assert_eq!(grid.capacity(), size!(0, 0));
    }

    #[test]
    fn grid_flip_vertically() {
        // [1, 2, 3] => [7, 8, 9]
        // [4, 5, 6]    [4, 5, 6]
        // [7, 8, 9]    [1, 2, 3]
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.flip_vertically();

        assert_eq!(grid.size(), size!(3, 3));
        assert_eq!(grid.capacity(), size!(3, 3));

        assert_eq!(grid[coord!(0, 0)], 7);
        assert_eq!(grid[coord!(1, 0)], 8);
        assert_eq!(grid[coord!(2, 0)], 9);
        assert_eq!(grid[coord!(0, 1)], 4);
        assert_eq!(grid[coord!(1, 1)], 5);
        assert_eq!(grid[coord!(2, 1)], 6);
        assert_eq!(grid[coord!(0, 2)], 1);
        assert_eq!(grid[coord!(1, 2)], 2);
        assert_eq!(grid[coord!(2, 2)], 3);

        // [1, 2, 3] => [4, 5, 6]
        // [4, 5, 6]    [1, 2, 3]
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6]]);

        grid.flip_vertically();

        assert_eq!(grid.size(), size!(3, 2));
        assert_eq!(grid.capacity(), size!(3, 2));

        assert_eq!(grid[coord!(0, 0)], 4);
        assert_eq!(grid[coord!(1, 0)], 5);
        assert_eq!(grid[coord!(2, 0)], 6);
        assert_eq!(grid[coord!(0, 1)], 1);
        assert_eq!(grid[coord!(1, 1)], 2);
        assert_eq!(grid[coord!(2, 1)], 3);

        // [1, 2] => [5, 6]
        // [3, 4]    [3, 4]
        // [5, 6]    [1, 2]
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4],
                                            vec![5, 6]]);

        grid.flip_vertically();

        assert_eq!(grid.size(), size!(2, 3));
        assert_eq!(grid.capacity(), size!(2, 3));

        assert_eq!(grid[coord!(0, 0)], 5);
        assert_eq!(grid[coord!(1, 0)], 6);
        assert_eq!(grid[coord!(0, 1)], 3);
        assert_eq!(grid[coord!(1, 1)], 4);
        assert_eq!(grid[coord!(0, 2)], 1);
        assert_eq!(grid[coord!(1, 2)], 2);

        // [1] => [1]
        let mut grid = Grid::from_rows(vec![vec![1]]);

        grid.flip_vertically();

        assert_eq!(grid.size(), size!(1, 1));
        assert_eq!(grid.capacity(), size!(1, 1));

        assert_eq!(grid[coord!(0, 0)], 1);

        // [] => []
        let mut grid = Grid::<()>::zero();

        grid.flip_vertically();

        assert_eq!(grid.size(), size!(0, 0));
        assert_eq!(grid.capacity(), size!(0, 0));
    }

    #[test]
    fn grid_rotate_left() {
        // [1, 2, 3] => [3, 6, 9]
        // [4, 5, 6]    [2, 5, 8]
        // [7, 8, 9]    [1, 4, 7]
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.rotate_left();

        assert_eq!(grid.size(), size!(3, 3));
        assert_eq!(grid.capacity(), size!(3, 3));

        assert_eq!(grid[coord!(0, 0)], 3);
        assert_eq!(grid[coord!(1, 0)], 6);
        assert_eq!(grid[coord!(2, 0)], 9);
        assert_eq!(grid[coord!(0, 1)], 2);
        assert_eq!(grid[coord!(1, 1)], 5);
        assert_eq!(grid[coord!(2, 1)], 8);
        assert_eq!(grid[coord!(0, 2)], 1);
        assert_eq!(grid[coord!(1, 2)], 4);
        assert_eq!(grid[coord!(2, 2)], 7);

        // [1, 2, 3] => [3, 6]
        // [4, 5, 6]    [2, 5]
        //              [1, 4]
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6]]);

        grid.rotate_left();

        assert_eq!(grid.size(), size!(2, 3));
        assert_eq!(grid.capacity(), size!(2, 3));

        assert_eq!(grid[coord!(0, 0)], 3);
        assert_eq!(grid[coord!(1, 0)], 6);
        assert_eq!(grid[coord!(0, 1)], 2);
        assert_eq!(grid[coord!(1, 1)], 5);
        assert_eq!(grid[coord!(0, 2)], 1);
        assert_eq!(grid[coord!(1, 2)], 4);

        // [1, 2] => [2, 4, 6]
        // [3, 4]    [1, 3, 5]
        // [5, 6]
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4],
                                            vec![5, 6]]);

        grid.rotate_left();

        assert_eq!(grid.size(), size!(3, 2));
        assert_eq!(grid.capacity(), size!(3, 2));

        assert_eq!(grid[coord!(0, 0)], 2);
        assert_eq!(grid[coord!(1, 0)], 4);
        assert_eq!(grid[coord!(2, 0)], 6);
        assert_eq!(grid[coord!(0, 1)], 1);
        assert_eq!(grid[coord!(1, 1)], 3);
        assert_eq!(grid[coord!(2, 1)], 5);

        // [1] => [1]
        let mut grid = Grid::from_rows(vec![vec![1]]);

        grid.rotate_left();

        assert_eq!(grid.size(), size!(1, 1));
        assert_eq!(grid.capacity(), size!(1, 1));

        assert_eq!(grid[coord!(0, 0)], 1);

        // [] => []
        let mut grid = Grid::<()>::zero();

        grid.rotate_left();

        assert_eq!(grid.size(), size!(0, 0));
        assert_eq!(grid.capacity(), size!(0, 0));
    }

    #[test]
    fn grid_rotate_right() {
        // [1, 2, 3] => [7, 4, 1]
        // [4, 5, 6]    [8, 5, 2]
        // [7, 8, 9]    [9, 6, 3]
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6],
                                            vec![7, 8, 9]]);

        grid.rotate_right();

        assert_eq!(grid.size(), size!(3, 3));
        assert_eq!(grid.capacity(), size!(3, 3));

        assert_eq!(grid[coord!(0, 0)], 7);
        assert_eq!(grid[coord!(1, 0)], 4);
        assert_eq!(grid[coord!(2, 0)], 1);
        assert_eq!(grid[coord!(0, 1)], 8);
        assert_eq!(grid[coord!(1, 1)], 5);
        assert_eq!(grid[coord!(2, 1)], 2);
        assert_eq!(grid[coord!(0, 2)], 9);
        assert_eq!(grid[coord!(1, 2)], 6);
        assert_eq!(grid[coord!(2, 2)], 3);

        // [1, 2, 3] => [4, 1]
        // [4, 5, 6]    [5, 2]
        //              [6, 3]
        let mut grid = Grid::from_rows(vec![vec![1, 2, 3],
                                            vec![4, 5, 6]]);

        grid.rotate_right();

        assert_eq!(grid.size(), size!(2, 3));
        assert_eq!(grid.capacity(), size!(2, 3));

        assert_eq!(grid[coord!(0, 0)], 4);
        assert_eq!(grid[coord!(1, 0)], 1);
        assert_eq!(grid[coord!(0, 1)], 5);
        assert_eq!(grid[coord!(1, 1)], 2);
        assert_eq!(grid[coord!(0, 2)], 6);
        assert_eq!(grid[coord!(1, 2)], 3);

        // [1, 2] => [5, 3, 1]
        // [3, 4]    [6, 4, 2]
        // [5, 6]
        let mut grid = Grid::from_rows(vec![vec![1, 2],
                                            vec![3, 4],
                                            vec![5, 6]]);

        grid.rotate_right();

        assert_eq!(grid.size(), size!(3, 2));
        assert_eq!(grid.capacity(), size!(3, 2));

        assert_eq!(grid[coord!(0, 0)], 5);
        assert_eq!(grid[coord!(1, 0)], 3);
        assert_eq!(grid[coord!(2, 0)], 1);
        assert_eq!(grid[coord!(0, 1)], 6);
        assert_eq!(grid[coord!(1, 1)], 4);
        assert_eq!(grid[coord!(2, 1)], 2);

        // [1] => [1]
        let mut grid = Grid::from_rows(vec![vec![1]]);

        grid.rotate_right();

        assert_eq!(grid.size(), size!(1, 1));
        assert_eq!(grid.capacity(), size!(1, 1));

        assert_eq!(grid[coord!(0, 0)], 1);

        // [] => []
        let mut grid = Grid::<()>::zero();

        grid.rotate_right();

        assert_eq!(grid.size(), size!(0, 0));
        assert_eq!(grid.capacity(), size!(0, 0));
    }

    #[test]
    fn grid_capacity() {
        let grid = Grid::<()>::zero();
        assert_eq!(grid.capacity(), size!(0, 0));

        let grid = Grid::from_rows(vec![vec![1, 2],
                                        vec![3, 4]]);
        assert_eq!(grid.capacity(), size!(2, 2));
    }

    #[test]
    fn grid_reserve() {
        let mut grid = Grid::<()>::zero();
        assert_eq!(grid.capacity(), size!(0, 0));

        grid.reserve(size!(3, 0));
        assert_eq!(grid.capacity(), size!(3, 0));

        grid.reserve(size!(0, 3));
        assert_eq!(grid.capacity(), size!(3, 3));

        grid.reserve(size!(2, 2));
        assert_eq!(grid.capacity(), size!(5, 5));
    }
}