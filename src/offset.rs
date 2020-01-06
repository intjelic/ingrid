// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

/// A two-dimensional offset
///
/// This structure defines a basic two-dimensional offset to shift values
/// during some operations on grid. It enables copy semantics to avoid handling
/// ownership and references throughout your code for no significant
/// performance. Also, in practice, you use the `offset!` macro helper to
/// instantiate offsets; this is to be consistent with the use of coordinates.
///
/// # Examples
///
/// ```
/// # use ingrid::{Offset, offset};
///
/// // Create a size to shift the elements of a grid.
/// let mut offset1 = offset!(-1, 1);
///
/// // Copy the offset into another variable.
/// let offset2 = offset1;
///
/// // The first variable is still accessible.
/// offset1.x = 1;
/// offset1.y = -1;
///
/// // Additionally, you could also type the following.
/// let offset3 = Offset::zero();
/// ```
///
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Offset {
    /// The offset value on the X axis.
    pub x: isize,

    /// The offset value on the Y axis.
    pub y: isize
}

impl Offset {
    /// Construct a new offset.
    ///
    /// This function constructs a new offset from given X and Y values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Offset;
    ///
    /// let offset = Offset::new(-1, 1);
    ///
    /// assert_eq!(offset.x, -1);
    /// assert_eq!(offset.y, 1);
    /// ```
    ///
    pub fn new(x: isize, y: isize) -> Offset {
        Offset { x, y }
    }

    /// Construct a zero offset.
    ///
    /// This function constructs a 'zero' offset which is a offset with both `x`
    /// and `y` values set at `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Offset;
    ///
    /// let offset = Offset::zero();
    ///
    /// assert_eq!(offset.x, 0);
    /// assert_eq!(offset.y, 0);
    /// ```
    ///
    pub fn zero() -> Offset {
        Offset { x: 0, y: 0 }
    }
}

/// An offset instantiation helper.
///
/// This macro helps instantiate offsets with a shorter syntax. Instead of
/// typing a full `Offset::new(x, y)`, one simply has to write `offset!(x, y)`
/// leading to more readable code.
///
/// # Examples
///
/// ```
/// # use ingrid::{Offset, offset};
/// assert_eq!(offset!(-1, 1), Offset::new(-1, 1));
/// ```
///
#[macro_export]
macro_rules! offset {
    ($x:expr, $y:expr) => {
        Offset::new($x, $y);
    };
}
