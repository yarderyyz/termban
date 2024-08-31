use ratatui::style::Color;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::colors::{get_color, TolColor};
use ndarray::Array2;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub enum Action {
    Quit,
    Move(Direction),
    Undo,
    Reset,
    None,
}

#[derive(Debug, Clone)]
pub enum Tile {
    Empty,
    Wall,
    Goal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub position: Coordinate,
}

#[derive(Debug, Clone)]
pub struct SokoBox {
    pub position: Coordinate,
}

#[derive(Debug, Clone)]
pub enum Entity {
    Player(Player),
    SokoBox(SokoBox),
}

impl Entity {
    pub fn get_position(&self) -> Coordinate {
        match self {
            Entity::Player(player) => player.position.clone(),
            Entity::SokoBox(soko_box) => soko_box.position.clone(),
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Entity::Player(_) => get_color(TolColor::PurRed),
            Entity::SokoBox(_) => get_color(TolColor::BriBlue),
        }
    }
}

type Board = Array2<Tile>;

#[derive(Debug, Clone)]
pub struct World {
    pub name: String,
    pub board: Board,
    pub entities: Vec<Entity>,
    #[allow(dead_code)]
    pub camera_position: Coordinate, // TODO: should I be on world or GameWindow for now?
}

impl World {
    pub fn is_tile_occupied(&self, coord: &Coordinate) -> bool {
        match self.board[[coord.y, coord.x]] {
            Tile::Wall => true,
            _ => {
                for ent in self.entities.iter() {
                    if ent.get_position() == *coord {
                        return true;
                    }
                }
                false
            }
        }
    }
}

impl Tile {
    pub fn color(&self) -> Option<Color> {
        match self {
            Tile::Wall => Some(get_color(TolColor::LigLightBlue)),
            Tile::Goal => Some(get_color(TolColor::BriGrey)),
            Tile::Empty => Some(get_color(TolColor::CstLigBlue)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameWindow {
    pub world: World,
}

#[derive(Debug, Clone)]
pub struct GlyphCell {
    glyph: char,
    fg: Option<Color>,
    bg: Option<Color>,
}
impl GlyphCell {
    pub fn new() -> GlyphCell {
        GlyphCell {
            glyph: ' ',
            fg: None,
            bg: None,
        }
    }
}

type GlyphCells = Array2<GlyphCell>;

#[derive(Debug, Clone)]
enum RenderItem {
    Board(Board),
    Entity(Entity),
}

#[derive(Debug, Clone)]
struct RenderNode {
    item: RenderItem,
    children: Option<Vec<RenderNode>>,
}

#[derive(Debug, Clone)]
struct RenderGraph {
    root: RenderNode,
}

fn generate_render_graph(world: &World) -> RenderGraph {
    let children = Some(
        world
            .entities
            .iter()
            .map(|ent| RenderNode {
                item: RenderItem::Entity(ent.clone()),
                children: None,
            })
            .collect(),
    );

    RenderGraph {
        root: RenderNode {
            item: RenderItem::Board(world.board.clone()),
            children,
        },
    }
}

fn glyphize_node(node: RenderNode, glyph_buffer: GlyphCells) -> GlyphCells {
    let glyph_buffer = match node.item {
        RenderItem::Board(board) => {
            // TODO: finish debugging
            let mut glyph_buffer = glyph_buffer.clone();

            for (yi, row) in board.outer_iter().enumerate() {
                for (xi, tile) in row.iter().enumerate() {
                    if xi % 2 == 0 {
                        glyph_buffer[[xi, yi]].glyph = '▀';
                        glyph_buffer[[xi, yi]].fg = tile.color();
                        // glyph_buffer[[xi, yi]].fg = Some(Color::Rgb(255, 0, 0));
                    } else {
                        glyph_buffer[[xi, yi]].glyph = '▀';
                        glyph_buffer[[xi, yi]].bg = tile.color();
                        // glyph_buffer[[xi, yi]].bg = Some(Color::Rgb(0, 0, 255));
                    }
                }
            }
            glyph_buffer
        }
        RenderItem::Entity(entity) => {
            // TODO: This logic is wrong
            let mut glyph_buffer = glyph_buffer.clone();
            let pos = entity.get_position();
            glyph_buffer[[pos.y, pos.x]].fg = Some(entity.color());
            glyph_buffer
        }
    };
    if let Some(children) = node.children {
        children
            .into_iter()
            .fold(glyph_buffer, |buffer, child| glyphize_node(child, buffer))
    } else {
        glyph_buffer
    }
}

/// Renders the current state of the game world into a pixel color array and character array
///
///
///
/// # Parameters
///
/// - `graph`: A reference to the `World` struct, which contains the game board and entities.
/// - `area`: A `Rect` specifying the area of the screen where the graph should be rendered.
///
/// # Details
///
/// - FILL ME OUT
///
/// # Examples
///
fn glyphize_graph(graph: RenderGraph, area: Rect) -> GlyphCells {
    let glyph_buffer = GlyphCells::from_elem(
        (area.height as usize, area.width as usize),
        GlyphCell::new(),
    );

    glyphize_node(graph.root, glyph_buffer)
}

pub fn blit(glyph_cells: GlyphCells, area: Rect, buf: &mut Buffer) {
    for (yi, row) in glyph_cells.outer_iter().enumerate() {
        for (xi, cell) in row.iter().enumerate() {
            let curs = &mut buf[(xi as u16 + area.x, yi as u16 + area.y)];
            curs.set_char(cell.glyph);
            if let Some(fg) = cell.fg {
                curs.set_fg(fg);
            }
            if let Some(bg) = cell.bg {
                curs.set_fg(bg);
            }
        }
    }
}

impl Widget for GameWindow {
    #[allow(clippy::cast_precision_loss)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let graph = generate_render_graph(&self.world);
        let glyph_buffer = glyphize_graph(graph, area);
        blit(glyph_buffer, area, buf);
    }
}
