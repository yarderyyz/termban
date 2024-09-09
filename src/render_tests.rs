#[cfg(test)]
use crate::{
    render::*,
    types::{Coordinate, GlyphCells},
};
#[cfg(test)]
use ndarray::Array2;

#[test]
fn coordinate_is_inside_the_bounds() {
    let glyph_cells_buffer: GlyphCells = Array2::default((5, 5));

    assert!(is_in_bounds(
        &Coordinate { x: 1, y: 1 },
        &glyph_cells_buffer
    ));

    assert!(is_in_bounds(
        &Coordinate { x: 4, y: 4 },
        &glyph_cells_buffer
    ));

    assert!(is_in_bounds(
        &Coordinate { x: 0, y: 4 },
        &glyph_cells_buffer
    ));
}

#[test]
fn coordinate_is_outside_the_bounds() {
    let glyph_cells_buffer: GlyphCells = Array2::default((5, 5));

    assert!(!is_in_bounds(
        &Coordinate { x: 100, y: 100 },
        &glyph_cells_buffer
    ));

    assert!(!is_in_bounds(
        &Coordinate { x: 4, y: 100 },
        &glyph_cells_buffer
    ));

    assert!(!is_in_bounds(
        &Coordinate { x: 100, y: 1 },
        &glyph_cells_buffer
    ));

    assert!(!is_in_bounds(
        &Coordinate { x: 5, y: 1 },
        &glyph_cells_buffer
    ));
}
