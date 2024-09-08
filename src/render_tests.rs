#[cfg(test)]
mod render_tests {
    use crate::{
        render::*,
        types::{Coordinate, GlyphCells},
    };
    use ndarray::Array2;

    #[test]
    fn coordinate_is_inside_the_bounds() {
        let glyph_cells_buffer: GlyphCells = Array2::default((5, 5));

        assert_eq!(
            true,
            is_in_bounds(&Coordinate { x: 1, y: 1 }, &glyph_cells_buffer)
        );

        assert_eq!(
            true,
            is_in_bounds(&Coordinate { x: 4, y: 4 }, &glyph_cells_buffer)
        );

        assert_eq!(
            true,
            is_in_bounds(&Coordinate { x: 0, y: 4 }, &glyph_cells_buffer)
        );
    }

    #[test]
    fn coordinate_is_outside_the_bounds() {
        let glyph_cells_buffer: GlyphCells = Array2::default((5, 5));

        assert_eq!(
            false,
            is_in_bounds(&Coordinate { x: 100, y: 100 }, &glyph_cells_buffer)
        );

        assert_eq!(
            false,
            is_in_bounds(&Coordinate { x: 4, y: 100 }, &glyph_cells_buffer)
        );

        assert_eq!(
            false,
            is_in_bounds(&Coordinate { x: 100, y: 1 }, &glyph_cells_buffer)
        );

        assert_eq!(
            false,
            is_in_bounds(&Coordinate { x: 5, y: 1 }, &glyph_cells_buffer)
        );
    }
}
