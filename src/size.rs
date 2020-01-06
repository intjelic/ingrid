// Copyright (c) 2020 - BytePlug
//
// This source file is part of Ingrid which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project
// directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

/// A two-dimensional size
///
/// This structure defines a basic two-dimensional size to specify the dimension
/// of grids. It enables copy semantics to avoid handling ownership and
/// references throughout your code for no significant performance. Also, in
/// practice, you use the `size!` macro helper to instantiate sizes; this is to
/// be consistent with the use of coordinates.
///
/// # Examples
///
/// ```
/// # use ingrid::{Size, size};
/// #
/// // Create a size to specify the dimension of a grid.
/// let mut size1 = size!(24, 42);
///
/// // Copy the size into another variable.
/// let size2 = size1;
///
/// // The first variable is still accessible.
/// size1.width = 42;
/// size1.height = 0;
///
/// // Additionally, you could also type the following.
/// let size3 = Size::zero();
/// ```
///
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Size {
    /// The width of the size.
    pub width: usize,

    /// The height of the size.
    pub height: usize
}

impl Size {
    /// Construct a new size.
    ///
    /// This function constructs a new coordinate from given width and height
    /// values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Size;
    /// #
    /// let size = Size::new(24, 42);
    ///
    /// assert_eq!(size.width, 24);
    /// assert_eq!(size.height, 42);
    /// ```
    ///
    pub fn new(width: usize, height: usize) -> Size {
        Size { width, height }
    }

    /// Construct a zero size.
    ///
    /// This function constructs a 'zero' size which is a coordinate with both
    /// `width` and `height` values set at `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ingrid::Size;
    /// #
    /// let size = Size::zero();
    ///
    /// assert_eq!(size.width, 0);
    /// assert_eq!(size.height, 0);
    /// ```
    ///
    pub fn zero() -> Size {
        Size { width: 0, height: 0 }
    }
}

/// A size instantiation helper.
///
/// This macro helps instantiate sizes with a shorter syntax. Instead of typing
/// a full `Size::new(width, height)`, one simply has to write
/// `size!(width, height)` leading to more readable code.
///
/// # Examples
///
/// ```
/// # use ingrid::{Size, size};
/// #
/// assert_eq!(size!(24, 42), Size::new(24, 42));
/// ```
///
#[macro_export]
macro_rules! size {
    ($width:expr, $height:expr) => {
        Size::new($width, $height);
    };
}