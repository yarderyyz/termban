use crate::sprites::get_player_sprite_4;
use crate::types::{
    Coordinate, Entity, GameWindow, GlyphCell, GlyphCells, RenderGraph, RenderItem,
    RenderNode, World, Zoom,
};

use ndarray::Array2;
use ratatui::style::Color;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub fn is_in_bounds<T>(position: &Coordinate, buffer: &Array2<T>) -> bool {
    let (height, width) = buffer.dim(); // Get the dimensions of the buffer

    position.x < width && position.y < height
}

fn render_pixels(
    pixel_size: usize,
    item: RenderItem,
    glyph_buffer: GlyphCells,
) -> GlyphCells {
    match item {
        RenderItem::Board(board) => {
            let mut glyph_buffer = glyph_buffer.clone();

            for (yi, row) in board.rows().into_iter().enumerate() {
                for (xi, tile) in row.iter().enumerate() {
                    if let Some(color) = tile.color() {
                        let pos = Coordinate { x: xi, y: yi };
                        glyph_buffer =
                            draw_square(color, pos, pixel_size, glyph_buffer.clone());
                    }
                }
            }
            glyph_buffer
        }
        RenderItem::Entity(entity) => {
            let pos = entity.get_position();
            draw_square(entity.color(), pos, pixel_size, glyph_buffer)
        }
    }
}

fn draw_square(
    color: Color,
    position: Coordinate,
    size: usize,
    glyph_buffer: GlyphCells,
) -> GlyphCells {
    let (x, y) = (position.x * size, position.y * size);
    let mut glyph_buffer = glyph_buffer.clone();
    for yi in y..(y + size) {
        for xi in x..(x + size) {
            // Each "pixel" is actually a single unicode character so in transforming from a grid
            // of pixels to a grid of GlyphCells the y position in the Glyph Grid is half of the y
            // position in the pixel grid
            let pos = Coordinate { x: xi, y: yi / 2 };
            if is_in_bounds(&pos, &glyph_buffer) {
                if yi % 2 == 0 {
                    glyph_buffer[pos.arr_index()].glyph = '▀';
                    glyph_buffer[pos.arr_index()].fg = Some(color);
                } else {
                    glyph_buffer[pos.arr_index()].bg = Some(color);
                }
            }
        }
    }
    glyph_buffer
}

fn render_sprites(item: RenderItem, glyph_buffer: GlyphCells) -> GlyphCells {
    match item {
        RenderItem::Board(board) => {
            let mut glyph_buffer = glyph_buffer.clone();

            for (yi, row) in board.rows().into_iter().enumerate() {
                for (xi, tile) in row.iter().enumerate() {
                    if let Some(color) = tile.color() {
                        let pos = Coordinate { x: xi, y: yi };
                        glyph_buffer = draw_square(color, pos, 4, glyph_buffer.clone());
                    }
                }
            }
            glyph_buffer
        }
        RenderItem::Entity(entity) => {
            let pos = entity.get_position();
            match entity {
                Entity::Player(_) => {
                    let mut glyph_buffer = glyph_buffer.clone();
                    let player_sprite = get_player_sprite_4();
                    for (yi, row) in player_sprite.chars.rows().into_iter().enumerate()
                    {
                        for (xi, pixel) in row.iter().enumerate() {
                            let index = [yi + (pos.y * 2), xi + (pos.x * 4)];
                            glyph_buffer[index].glyph = pixel.char;
                            if pixel.fg.is_some() {
                                glyph_buffer[index].fg = pixel.fg;
                            }
                            if pixel.bg.is_some() {
                                glyph_buffer[index].bg = pixel.bg;
                            }
                        }
                    }
                    glyph_buffer
                }
                entity => draw_square(entity.color(), pos, 4, glyph_buffer),
            }
        }
    }
}

/// Generates a `RenderGraph` from the provided `World` struct.
///
/// # Parameters
///
/// - `world`: A reference to a `World` struct, which contains the game's state, including the board and entities.
///
/// # Returns
///
/// A `RenderGraph` struct that represents the entire scene to be rendered. The root node of the graph corresponds to
/// the game's board, and its children correspond to the entities present in the world.
///
/// # Details
///
/// This function creates a hierarchical `RenderGraph` that represents the current state of the game world. The root
/// node of the graph contains a `RenderItem::Board`, which represents the game board. The root node's children are
/// `RenderNode`s that each contain a `RenderItem::Entity`, representing the entities in the world. Each entity is
/// cloned from the `world` and encapsulated in a `RenderNode` without further children (i.e., `children` is `None`).
///
/// This graph structure allows for organized and flexible rendering of the game world. By constructing the scene
/// as a tree of `RenderNode`s, it becomes easier to traverse and render the scene in a systematic way, ensuring
/// that the board is rendered first, followed by the entities in the correct order.
///
/// This function is typically called before rendering to prepare the data needed to generate a visual representation
/// of the game state.
///
/// # Examples
///
/// ```rust
/// let world = World::new(); // Assume World::new initializes a game world
/// let render_graph = generate_render_graph(&world);
///
/// // `render_graph` can now be passed to a rendering function
/// let area = Rect::new(0, 0, 80, 24); // Example rendering area
/// let glyph_buffer = glypherize_graph(render_graph, area, render_fn);
/// ```
fn generate_render_graph(world: &World) -> RenderGraph {
    let children = world
        .entities
        .iter()
        .map(|ent| RenderNode {
            item: RenderItem::Entity(ent.clone()),
            children: None,
        })
        .collect();

    RenderGraph {
        root: RenderNode {
            item: RenderItem::Board(world.board.clone()),
            children: Some(children),
        },
    }
}

/// Renders a node in the render graph into a new buffer
///
/// # Parameters
///
/// - `node`: A `RenderNode` to be rendered.
/// - `glyph_buffer`: A `GlyphBuffer` containing previous buffer to render over.
/// - `render_fn`: A reference to a render fn takes an item and returns a new glyph_buffer.
///
/// # Details
///
/// Currently this method is pure function that takes a buffer and returns a new buffer.
/// This probably isn't how we want to do this long term but for now it's a simple approach.
/// Eventually I'd like to do a version that takes and renders into a `&mut GlyphCells`
/// buffer instead. And compare the performance of the pure functional approach to something
/// closer to how this would be done in an optimized engine.
///
/// # Examples
///
fn glypherize_node<F>(
    node: RenderNode,
    glyph_buffer: GlyphCells,
    render_fn: &F,
) -> GlyphCells
where
    F: Fn(RenderItem, GlyphCells) -> GlyphCells,
{
    let glyph_buffer = render_fn(node.item, glyph_buffer);
    if let Some(children) = node.children {
        children.into_iter().fold(glyph_buffer, |buffer, child| {
            glypherize_node(child, buffer, render_fn)
        })
    } else {
        glyph_buffer
    }
}

/// Takes a `RenderGraph` and renders it into a buffer for display
///
/// # Parameters
///
/// - `graph`: A `RenderGraph` struct, which represents the scene to render.
/// - `area`: A `Rect` specifying the area of the screen where the graph should be rendered.
/// - `render_fn`: A reference to a render fn takes an item and returns a new glyph_buffer.
///
/// # Details
///
/// This function serves as the entry point for rendering a complex scene represented by a `RenderGraph`
/// into a grid of cells (`GlyphCells`) that can be displayed on a terminal or another unicode grid-based rendering
/// system. The function starts by initializing an empty `GlyphCells` buffer with dimensions matching the
/// specified `area`. It then recursively traverses the nodes of the `RenderGraph`, applying the provided
/// `render_fn` to each `RenderItem` encountered. The `render_fn` is responsible for rendering each individual
/// item onto the `GlyphCells` buffer. Currently this is a purely functional process. In the future
/// this should be replaced with a mutable model for performance reasons.
///
/// The final `GlyphCells` buffer, containing the fully rendered scene, is returned to the caller. This
/// approach allows for flexible rendering strategies by passing different `render_fn` implementations
/// depending on the desired rendering behavior (e.g., rendering at different resolutions, applying
/// different styles, etc.).
///
/// # Examples
///
fn glypherize_graph<F>(graph: RenderGraph, area: Rect, render_fn: F) -> GlyphCells
where
    F: Fn(RenderItem, GlyphCells) -> GlyphCells,
{
    let glyph_buffer = GlyphCells::from_elem(
        (area.height as usize, area.width as usize),
        GlyphCell::new(),
    );

    glypherize_node(graph.root, glyph_buffer, &render_fn)
}

/// Implements the `Widget` trait for the `GameWindow` struct, allowing it to be rendered within
/// a specified area on the screen using a given buffer. The rendering is performed based on the
/// zoom level of the game, with different rendering strategies applied depending on the selected zoom.
///
/// # Parameters
///
/// - `area`: A `Rect` that defines the portion of the screen where the game window should be rendered.
/// - `buf`: A mutable reference to a `Buffer`, which will be used to store the rendered output.
///
/// # Details
///
/// - The method begins by generating a `RenderGraph` from the current state of the game world.
///   TODO: this should be reworked so that the render graph is only updated when the scene changes.
/// - Based on the zoom level (`Close`, `Middle`, or `Far`), the appropriate rendering function
///   (`render_4`, `render_2`, or `render_1`) is selected and applied to convert the `RenderGraph`
///   into a `GlyphCells` buffer.
/// - The `GlyphCells` buffer is then iterated over, and each `GlyphCell` in the buffer is drawn onto
///   the `buf` at the corresponding coordinates within the specified `area`.
/// - If a `CharPixel` has a foreground color (`fg`) or background color (`bg`), these colors are
///   applied to the respective cells in the buffer.
///
/// # Notes
///
/// - The method handles the precision loss clippy warning using `#[allow(clippy::cast_precision_loss)]`,
///   which is relevant when converting `usize` values (used for indexing) to `u16` for setting buffer coordinates.
///
/// # Examples
///
/// ```rust
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 25));
/// let game_window = GameWindow::new(world);
/// game_window.render(Rect::new(0, 0, 80, 25), &mut buffer);
/// ```
impl Widget for GameWindow {
    #[allow(clippy::cast_precision_loss)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let graph = generate_render_graph(&self.world);
        let glyph_buffer = match self.zoom {
            Zoom::Close => glypherize_graph(graph, area, render_sprites),
            Zoom::Middle => {
                glypherize_graph(graph, area, |item, buf| render_pixels(2, item, buf))
            }
            Zoom::Far => {
                glypherize_graph(graph, area, |item, buf| render_pixels(1, item, buf))
            }
        };

        for (yi, row) in glyph_buffer.rows().into_iter().enumerate() {
            for (xi, cell) in row.iter().enumerate() {
                let curs = &mut buf[(xi as u16 + area.x, yi as u16 + area.y)];
                curs.set_char(cell.glyph);
                if let Some(fg) = cell.fg {
                    curs.set_fg(fg);
                }
                if let Some(bg) = cell.bg {
                    curs.set_bg(bg);
                }
            }
        }
    }
}
