// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! # Dynamic two-dimensional array with its algorithms
//!
//! This crate provides a self-contained complete set of features to work with
//! **grids**, which is simply another term for **two-dimensional arrays**.
//!
//! Unlike regular arrays (or vectors), a grid cannot guarantee contiguity of
//! the memory from both rows and columns perspectives at the same time.
//! Therefore, we cannot consistently work with slices (which strictly are for contiguous
//! sequences) and benefits from their useful features. The solution is to
//! emulate slices for rows and columns, and implement their functionalities
//! whenever they make sense in order to work with an unified interface, free
//! from the internal representation of the grid. Iterators are also implemented
//! and adapted to iterate over the grids, their rows, columns and cells, just
//! like one would expect to do it with regular arrays.
//!
//! But this crate isn't limited to the emulation of slices for rows and
//! columns. It attempts to provide a standard container and its common algorithms, with most
//! importantly, a consistent interface following the STL conventions, that is
//! generic and good enough for most use cases. It's documented from tail to
//! head and has complete test coverage.
//!
//! ## Quick preview
//!
//! Before digging in and have a look at each concept separately, here is a
//! snippet to get an idea of what it looks like to work with **Ingrid**.
//!
//! ```rust
//! use ingrid::{Coordinate, Size};
//! use ingrid::Grid;
//! use ingrid::GridIterator;
//! use ingrid::{coord, size}; // Macros to shorten the syntax
//!
//! // Create a grid with enough allocated memory to contain 9 elements.
//! let mut grid = Grid::<char>::with_capacity(size!(3, 3));
//!
//! // Resize the grid to be 2x2 and fill it with a default value.
//! grid.resize(size!(2, 3), '😞');
//!
//! // Change the content of the grid with the direct accessors.
//! grid[coord!(0, 0)] = '😄'; // Top-left element (first element)
//! grid[coord!(1, 2)] = '😄'; // Bottom-right element (last element)
//!
//! // Insert a column right in the middle.
//! grid.insert_column(1, vec!['😮', '😮', '😮']);
//!
//! // Iterate over the elements of the last row
//! for (coordinate, emoticon) in grid.row(2).iterator().enumerate_coordinate() {
//!     println!("Emoticon at {:?} is {}", coordinate, emoticon);
//! }
//! ```
//!
//! From now, you can either jump right in the documentation, or read this short
//! introduction.
//!
//! ## Coordinates and sizes
//!
//! Because the notion of "coordinate" and "size" are intrinsic to the grid,
//! this crate defines two very simple structures, `Coordinate` and `Size` to
//! deal with those kind of values.
//!
//! ```
//! # use ingrid::{Coordinate, Size};
//! #
//! let coordinate = Coordinate::new(0, 0); // The top-left corner of a grid.
//! // ... or ...
//! let coordinate = Coordinate::zero();
//!
//! let size = Size::new(3, 3);
//! ```
//!
//! You will notice they're a little bit verbose, and this is why macros are
//! there to have a shorter syntax.
//!
//! ```
//! # use ingrid::{Coordinate, Size, coord, size};
//! #
//! let coordinate = coord!(0, 0);
//! let size = size!(0, 0);
//! ```
//!
//! It's worth nothing that their type is `usize` and `isize` because they're
//! pointers to memory location and that they have the **copy semantic** enabled
//! because memory usage is not a concern.
//!
//! ```
//! # use ingrid::{Coordinate, coord};
//! #
//! let coord = coord!(0, 0);
//! let copy_coord = coord; // It creates a copy.
//! println!("coord {:?}", coord); // The proof is the initial variable is still accessible.
//! ```
//!
//! Additionally, there is also the `Offset` structure to use when you resize
//! a grid and want its content to be shifted by a given number of rows and
//! columns.
//!
//! ```
//! # use ingrid::{Offset, offset};
//! #
//! let offset = Offset::new(-1, 1);
//! // ... or ...
//! let offset = offset!(-1, 1);
//! ```
//!
//! ## Grid and its elements
//!
//! Grids are made of **rows**, **columns** and **cells**, which are widespread
//! terms and need no explanation. However, this crate distinguishes **cells**
//! and **elements** as the former is an intermediary construct to access the
//! elements of the grid.
//!
//! Just like vectors, the grid owns the elements but unlike vectors, they're
//! indexed with coordinates.
//!
//! ```
//! # use ingrid::{Coordinate, Size, Grid, coord, size};
//! #
//! let mut grid = Grid::with_size(size!(3, 3), 0);
//! grid[coord!(1, 1)] = 42;
//! ```
//!
//! A grid can be filled with a given value, resized and cleared out.
//!
//! ```
//! # use ingrid::{Size, Grid, size};
//! #
//! let mut grid = Grid::with_size(size!(3, 3), 0);
//!
//! // Fill the grid with a given value, replacing (and dropping) all existing elements.
//! grid.fill(42);
//!
//! // Resize the grid, and fill the new space with a given value.
//! grid.resize(size!(5, 5), 1);
//!
//! // Clear the grid, removing all elements and setting the grid size to 0.
//! grid.clear();
//! assert_eq!(grid.size(), size!(0, 0));
//! ```
//!
//! Note that if the width or height of the grid is equal to 0, it's said to be
//! an "empty" grid. Therefore, a grid with size (3, 0) is perfectly valid but
//! will contain no element.
//!
//! ```
//! # use ingrid::{Size, Grid, size};
//! #
//! let mut grid = Grid::zero();
//! assert_eq!(grid.size(), size!(0, 0));
//!
//! // The grid has a width, but because height is zero, it still holds no element.
//! grid.resize(size!(3, 0), 42);
//! assert_eq!(grid.size(), size!(3, 0));
//!
//! // Now it's holding 3 elements.
//! grid.insert_row(0, vec![1, 2, 3]);
//! assert_eq!(grid.size(), size!(3, 1));
//! ```
//!
//! For more complex algorithm, using cells instead of directly accessing
//! elements can be advantageous as they retain their coordinates and comes with
//! an handy interface to do stuff like surveying the adjacent cells.
//!
//! ## Capacity and reallocation
//!
//! Just like vectors has the notion of capacity, this crate also offers you
//! similar control over the allocated memory of grids that grows and shrinks.
//!
//! The capacity of a grid is the amount of space allocated for any future
//! elements that will be added. This is not to be confused with the size of a
//! grid, which specifies the number of actual elements within the grid. If a
//! grid's size exceeds its capacity, its capacity will automatically be
//! increased, but its elements will have to be reallocated.
//!
//! For example, a grid with capacity (10, 10) and size (0, 0) would be an empty
//! grid with space for 100 more elements. Inserting rows and columns onto the
//! grid will not change its capacity or cause reallocation to occur. However,
//! if the grid's size is increased to (11, 10) or (10, 11), it will have to
//! reallocate, which can be slow. For this reason, it is recommended to use
//! `Grid<T>::with_capacity()` whenever possible to specify how big the grid is
//! expected to get.
//!
//! ## Rows and columns
//!
//! Rows and columns are slice-like objects for the grid. They come in two
//! versions, immutable and immutable and can be constructed from the grid
//! itself.
//!
//! ```
//! # use ingrid::Grid;
//! #
//! let mut grid = Grid::from_rows(vec![vec![1, 0, 3],
//!                                     vec![0, 0, 0],
//!                                     vec![7, 0, 9]]);
//!
//! // Fix the grid to be 1, 2, 3, 4, 5, 6, 7, 8, 9.
//! {
//!     let mut row = grid.row_mut(1);
//!     row[0] = 4;
//!     row[1] = 5;
//!     row[2] = 6;
//! }
//!
//! {
//!     let mut column = grid.column_mut(1);
//!     column[0] = 2;
//!     column[2] = 8;
//! }
//!
//! let row = grid.row(1);
//! assert_eq!(row.values(), vec![&4, &5, &6]);
//!
//! let column = grid.column(1);
//! assert_eq!(column.values(), vec![&2, &5, &8]);
//! ```
//!
//! As shown per the example, you need a mutable version of the row or column if
//! you want to make changes to it.
//!
//! ## Set of iterators
//!
//! You will legitimately be iterating grids in all sort of ways and iterators
//! are there for that.
//!
//! ```
//! # use ingrid::Grid;
//! #
//! let grid = Grid::from_rows(vec![vec![1, 2],
//!                                 vec![3, 4]]);
//!
//! let mut iterator = grid.iterator();
//! assert_eq!(iterator.next(), Some(&1));
//! assert_eq!(iterator.next(), Some(&2));
//! assert_eq!(iterator.next(), Some(&3));
//! assert_eq!(iterator.next(), Some(&4));
//! assert_eq!(iterator.next(), None);
//! ```
//!
//! Below is a snippet showing how to iterate over specific rows and columns.
//!
//! ```
//! # use ingrid::Grid;
//! #
//! let mut grid = Grid::from_rows(vec![vec![1, 2],
//!                                     vec![3, 4]]);
//!
//! let mut row_iterator = grid.row(0).iterator();
//! assert_eq!(row_iterator.next(), Some(&1));
//! assert_eq!(row_iterator.next(), Some(&2));
//! assert_eq!(row_iterator.next(), None);
//!
//! let mut column_iterator = grid.column(1).iterator();
//! assert_eq!(column_iterator.next(), Some(&2));
//! assert_eq!(column_iterator.next(), Some(&4));
//! assert_eq!(column_iterator.next(), None);
//! ```
//!
//! The `Enumerate` iterator adaptor comes in handy to yield the index of a
//! slice. However, the index of a grid is a coordinate, therefore, use the
//! provided `EnumerateCoordinate` iterator adaptor to yield coordinate of the
//! elements during your iterations.
//!
//! ```
//! # use ingrid::Grid;
//! #
//! # let mut grid = Grid::from_rows(vec![vec![1, 2],
//! #                                     vec![3, 4]]);
//! #
//! for (coordinate, value) in grid.iterator().enumerate() {
//!     // Now you also have access to the coordinate of the elements; you don't
//!     // have to keep track of it yourself.
//!     println!("Value of element at {:?} is {}", coordinate, value);
//! }
//! ```
//!
//! ## The cell intermediary accessor
//!
//! This part of the crate isn't implemented yet.
//!
#[macro_use]
mod coordinate;
#[macro_use]
mod size;
#[macro_use]
mod offset;

mod grid;
mod row;
mod row_mut;
mod column;
mod column_mut;
mod cell;

mod grid_iterator;
mod iterator_grid;
mod iterator_row;
mod iterator_column;
mod enumerate_coordinate;

pub use coordinate::Coordinate;
pub use size::Size;
pub use offset::Offset;

pub use grid::Grid;
pub use row::Row;
pub use row_mut::RowMut;
pub use column::Column;
pub use column_mut::ColumnMut;
pub use cell::Cell;

pub use grid_iterator::GridIterator;
pub use iterator_grid::IteratorGrid;
pub use iterator_row::IteratorRow;
pub use iterator_column::IteratorColumn;
pub use enumerate_coordinate::EnumerateCoordinate;