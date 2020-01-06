# Ingrid

Ingrid is a self-contained crate that provides a STL-like container for dynamic two-dimensional arrays for the Rust programming language. It comes with
algorithms and it's expected to be generic and good enough for most use cases;
it was initially written to implement image pixels and grid-based games.

**Features**

- A concise dynamic grid structure
- Slice-like rows and columns
- Useful set of iterators and adaptors
- Smart implementation with capacity-like feature
- Ready-to-use common algorithms
- Complete code coverage
- Extensively documented

It's distributed under the MIT license. Feel free to use the way you like as
long as you keep the license around if you reuse the code.

## Quick preview

To get a rough idea of what it is like to work with **Ingrid**, have a look at
the following snipped.

```rust
use ingrid::{Coordinate, Size};
use ingrid::Grid;
use ingrid::GridIterator;
use ingrid::{coord, size}; // Macros to shorten the syntax

// Create a grid with enough allocated memory to contain 9 elements.
let mut grid = Grid::<char>::with_capacity(size!(3, 3));

// Resize the grid to be 2x2 and fill it with a default value.
grid.resize(size!(2, 3), '😞');

// Change the content of the grid with the direct accessors.
grid[coord!(0, 0)] = '😄'; // Top-left element (first element)
grid[coord!(1, 2)] = '😄'; // Bottom-right element (last element)

// Insert a column right in the middle.
grid.insert_column(1, vec!['😮', '😮', '😮']);

// Iterate over the elements of the last row
for (coordinate, emoticon) in grid.row(2).iterator().enumerate_coordinate() {
    println!("Emoticon at {:?} is {}", coordinate, emoticon);
}
```

Next step is to dig in the documentation, which comes with an excellent
introduction by the way.

## More information

**Website:** https://www.intjelic.me/project/ingrid
**Repository:** https://github.com/intjelic/ingrid
**Crate:** https://crates.io/crates/ingrid
**Documentation:** https://docs.rs/ingrid
**Author:** Jonahan De Wachter (dewachter.jonathan[at]gmail[dot]com)