#[cfg(test)]
mod tests {
    use crate::{soko_loader::*, types::Tile};
    use ndarray::array;
    use ndarray::Array2;

    #[test]
    fn test_cull_single_floor_tile_on_edge() {
        // Create a 3x3 board with Floor tiles
        let mut board = Array2::from_elem((3, 3), Tile::Floor);

        // Cull the tile at position (0, 1) which is on the top edge
        cull_tiles([0, 1], &mut board);

        // The tile at (0,1) should become Empty
        assert_eq!(board[[0, 1]], Tile::Empty);
    }

    #[test]
    fn test_cull_tile_with_adjacent_empty() {
        // Create a 3x3 board with Floor tiles
        let mut board = Array2::from_elem((3, 3), Tile::Floor);

        // Set one tile to Empty
        board[[1, 1]] = Tile::Empty;

        // Cull the tile at position (1, 0) which is adjacent to an Empty tile
        cull_tiles([1, 0], &mut board);

        // The tile at (1, 0) should become Empty
        assert_eq!(board[[1, 0]], Tile::Empty);
    }

    #[test]
    fn test_cull_recursively_adjacent_tiles() {
        // Create a 3x3 board with Floor tiles
        let mut board = Array2::from_elem((3, 3), Tile::Floor);

        // Set a tile on the edge to Empty
        board[[0, 1]] = Tile::Empty;

        // Cull the tile at position (1, 1), which is adjacent to (0,1)
        cull_tiles([1, 1], &mut board);

        // The tile at (1,1) and its adjacent tiles should become Empty recursively
        assert_eq!(board[[1, 1]], Tile::Empty);
        assert_eq!(board[[1, 2]], Tile::Empty);
        assert_eq!(board[[2, 1]], Tile::Empty);
        assert_eq!(board[[1, 0]], Tile::Empty);
    }

    #[test]
    fn test_cull_does_not_affect_walls_and_goals() {
        // Create a 3x3 board with Floor tiles
        let mut board = Array2::from_elem((3, 3), Tile::Floor);

        // Place a Wall and a Goal on the board
        board[[1, 1]] = Tile::Wall;
        board[[1, 2]] = Tile::Goal;

        // Cull starting from position (0, 1)
        cull_tiles([0, 1], &mut board);

        // The Wall and Goal should remain unaffected
        assert_eq!(board[[1, 1]], Tile::Wall);
        assert_eq!(board[[1, 2]], Tile::Goal);

        // Adjacent Floor tiles should be culled to Empty
        assert_eq!(board[[0, 1]], Tile::Empty);
        assert_eq!(board[[0, 0]], Tile::Empty);
        assert_eq!(board[[0, 2]], Tile::Empty);
    }

    #[test]
    fn test_cull_no_effect_on_interior_tile_with_no_adjacent_empty_or_edge() {
        // Create a 5x5 board with Floor tiles
        let mut board = Array2::from_elem((5, 5), Tile::Floor);

        // Cull starting from position (2, 2) (center tile)
        cull_tiles([2, 2], &mut board);

        // Since the center tile is not adjacent to any Empty tiles or edges, it should remain Floor
        assert_eq!(board[[2, 2]], Tile::Floor);
    }

    #[test]
    fn test_cull_edge_tiles_cause_recursive_culling_inwards() {
        // Create a 5x5 board with Floor tiles
        let mut board = Array2::from_elem((5, 5), Tile::Floor);

        // Cull starting from position (0, 2) (top edge)
        cull_tiles([0, 2], &mut board);

        // Tiles should be culled recursively inwards from the edge
        // The tiles in the first row should be Empty
        for x in 0..5 {
            assert_eq!(board[[0, x]], Tile::Empty);
        }

        // The adjacent tiles should be culled
        assert_eq!(board[[1, 2]], Tile::Empty);
        assert_eq!(board[[2, 2]], Tile::Empty);
    }

    #[test]
    fn test_cull_tiles_with_wrapping_sub_at_zero_index() {
        // Create a 3x3 board with Floor tiles
        let mut board = Array2::from_elem((3, 3), Tile::Floor);

        // Cull starting from position (0, 0) (top-left corner)
        cull_tiles([0, 0], &mut board);

        // Ensure that the tiles are culled correctly without index underflow
        assert_eq!(board[[0, 0]], Tile::Empty);
        assert_eq!(board[[0, 1]], Tile::Empty);
        assert_eq!(board[[1, 0]], Tile::Empty);
    }

    #[test]
    fn test_cull_empty_board() {
        // Create an empty board (all tiles are Empty)
        let mut board = Array2::from_elem((3, 3), Tile::Empty);

        // Apply culling
        cull_outer_tiles(&mut board);

        // Expected board remains the same
        let expected = Array2::from_elem((3, 3), Tile::Empty);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_cull_full_floor_board() {
        // Create a board where all tiles are Floor
        let mut board = Array2::from_elem((3, 3), Tile::Floor);

        // Apply culling
        cull_outer_tiles(&mut board);

        // Expected board after culling recursively
        let expected = Array2::from_elem((3, 3), Tile::Empty);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_cull_with_walls_and_goals() {
        // Create a board with Walls and Goals on the edges and Floors inside
        let mut board = array![
            [Tile::Wall, Tile::Goal, Tile::Wall],
            [Tile::Floor, Tile::Floor, Tile::Floor],
            [Tile::Wall, Tile::Goal, Tile::Wall],
        ];

        // Apply culling
        cull_outer_tiles(&mut board);

        // Expected board after culling recursively
        let expected = array![
            [Tile::Wall, Tile::Goal, Tile::Wall],
            [Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Wall, Tile::Goal, Tile::Wall],
        ];

        assert_eq!(board, expected);
    }

    #[test]
    fn test_cull_partial_floor_edges() {
        // Create a board with some Floor tiles on the edges connected to inner floors
        let mut board = array![
            [Tile::Empty, Tile::Floor, Tile::Empty],
            [Tile::Floor, Tile::Floor, Tile::Floor],
            [Tile::Empty, Tile::Floor, Tile::Empty],
        ];

        // Apply culling
        cull_outer_tiles(&mut board);

        // Expected board after culling recursively
        let expected = array![
            [Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty],
        ];

        assert_eq!(board, expected);
    }

    #[test]
    fn test_cull_larger_board() {
        // Create a 5x5 board with Floor tiles
        let mut board = Array2::from_elem((5, 5), Tile::Floor);

        // Set some inner tiles to Wall and Goal
        board[[2, 2]] = Tile::Wall;
        board[[1, 3]] = Tile::Goal;

        // Apply culling
        cull_outer_tiles(&mut board);

        // Expected board after culling recursively
        let mut expected = Array2::from_elem((5, 5), Tile::Empty);
        expected[[2, 2]] = Tile::Wall;
        expected[[1, 3]] = Tile::Goal;

        assert_eq!(board, expected);
    }

    #[test]
    fn test_cull_no_floor_on_edges() {
        // Create a board with no Floor tiles on the edges
        let mut board = array![
            [Tile::Wall, Tile::Empty, Tile::Wall],
            [Tile::Empty, Tile::Floor, Tile::Empty],
            [Tile::Wall, Tile::Empty, Tile::Wall],
        ];
        // Expected board remains the same
        let expected = board.clone();

        // Apply culling
        cull_outer_tiles(&mut board);

        assert_eq!(board, expected);
    }

    #[test]
    fn test_cull_inner_floor_tile_not_connected_to_edge() {
        // Create a board with an inner floor tile completely enclosed by walls
        let mut board = array![
            [Tile::Wall, Tile::Wall, Tile::Wall],
            [Tile::Wall, Tile::Floor, Tile::Wall],
            [Tile::Wall, Tile::Wall, Tile::Wall],
        ];
        // Expected board remains the same since the inner Floor tile is not connected to the edge
        let expected = board.clone();

        // Apply culling
        cull_outer_tiles(&mut board);

        assert_eq!(board, expected);
    }

    #[test]
    fn test_cull_complex_pattern() {
        // Create a complex board pattern with Floors connected to the edge
        #[rustfmt::skip]
        let mut board = array![
            [Tile::Floor, Tile::Wall, Tile::Floor, Tile::Wall, Tile::Floor],
            [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
            [Tile::Floor, Tile::Floor, Tile::Goal, Tile::Floor, Tile::Floor],
            [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
            [Tile::Floor, Tile::Wall, Tile::Floor, Tile::Wall, Tile::Floor],
        ];

        // Apply culling
        cull_outer_tiles(&mut board);

        // Expected board after culling recursively
        #[rustfmt::skip]
        let expected = array![
            [Tile::Empty, Tile::Wall, Tile::Empty, Tile::Wall, Tile::Empty],
            [Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            [Tile::Empty, Tile::Empty, Tile::Goal, Tile::Empty, Tile::Empty],
            [Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            [Tile::Empty, Tile::Wall, Tile::Empty, Tile::Wall, Tile::Empty],
        ];

        assert_eq!(board, expected);
    }

    #[test]
    fn test_cull_tiles_connected_to_edge() {
        // Create a board where floor tiles are connected to the edge
        #[rustfmt::skip]
        let mut board = array![
            [Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor],
            [Tile::Floor, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Floor],
            [Tile::Floor, Tile::Wall, Tile::Goal, Tile::Wall, Tile::Floor],
            [Tile::Floor, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Floor],
            [Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor],
        ];

        // Apply culling
        cull_outer_tiles(&mut board);

        // Expected board after culling recursively
        #[rustfmt::skip]
        let expected = array![
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Empty],
            [Tile::Empty, Tile::Wall, Tile::Goal, Tile::Wall, Tile::Empty],
            [Tile::Empty, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        ];

        assert_eq!(board, expected);
    }

    #[test]
    fn test_cull_tiles_not_connected_to_edge() {
        // Create a board where inner floor tiles are isolated from the edge
        #[rustfmt::skip]
        let mut board = array![
            [Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
            [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
            [Tile::Wall, Tile::Floor, Tile::Goal, Tile::Floor, Tile::Wall],
            [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
            [Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        ];
        // Expected board remains the same since the Floor tiles are not connected to the edge
        let expected = board.clone();

        // Apply culling
        cull_outer_tiles(&mut board);

        assert_eq!(board, expected);
    }
}
