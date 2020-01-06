// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

/// A two-dimensional coordinate
///
/// This structure defines a basic two-dimensional coordinate to index grids. It
/// enables copy semantics to avoid handling ownership and references
/// throughout your code for no significant performance. Also, in practice, you
/// use the `coord!` macro helper to instantiate coordinates.
///
/// # Examples
///
/// ```
/// # use ingrid::{Coordinate, coord};
/// #
/// // Create a coordinate to index the top-left element of a grid.
/// let mut coord1 = coord!(0, 0);
///
/// // Copy the coordinate into another variable.
/// let coord2 = coord1;
///
/// // The first variable is still accessible.
/// coord1.x = 42;
/// coord1.y += 1;
///
/// // Additionally, you could also type the following.
/// let coord3 = Coordinate::zero();
/// ```
///
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coordinate {
    /// The coordinate on the X axis.
    pub x: usize,

    /// The coordinate on the Y axis.
    pub y: usize
}

impl Coordinate {
    /// Construct a new coordinate.
    ///
    /// This function constructs a new coordinate from given X and Y values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Coordinate;
    /// #
    /// let coord = Coordinate::new(0, 42);
    ///
    /// assert_eq!(coord.x, 0);
    /// assert_eq!(coord.y, 42);
    /// ```
    ///
    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }

    /// Construct a zero coordinate.
    ///
    /// This function constructs a 'zero' coordinate which is a coordinate with
    /// both `x` and `y` values set at `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Coordinate;
    /// #
    /// let coord = Coordinate::zero();
    ///
    /// assert_eq!(coord.x, 0);
    /// assert_eq!(coord.y, 0);
    /// ```
    ///
    pub fn zero() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }
}

/// A coordinate instantiation helper.
///
/// This macro helps instantiate coordinates with a shorter syntax. Instead of
/// typing a full `Coordinate::new(x, y)`, one simply has to write
/// `coord!(x, y)` leading to more readable code.
///
/// # Examples
///
/// ```
/// # use ingrid::{Coordinate, coord};
/// assert_eq!(coord!(0, 0), Coordinate::new(0, 0));
/// ```
///
#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr) => {
        Coordinate::new($x, $y);
    };
}