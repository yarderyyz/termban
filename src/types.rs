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
    // pub camera_position: Coordinate, // TODO: should I be on world or GameWindow for now?
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

// TODO: idea's for better name here?
struct UnicodePixels {
    colors: Array2<Option<Color>>,
    chars: Array2<Option<char>>,
}

impl UnicodePixels {
    pub fn new(
        colors: Array2<Option<Color>>,
        chars: Array2<Option<char>>,
    ) -> UnicodePixels {
        UnicodePixels { colors, chars }
    }
}

/// Renders the current state of the game world into a pixel color array and character array
///
/// This function takes the current game world (`graph`) it is currently really chince and
/// just uses the world rather than generating a render graph.
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
fn render_graph(graph: &World, area: Rect) -> UnicodePixels {
    let _ = area.width;
    let _ = area.height;

    let mut colors = graph.board.map(|tile| tile.color());
    let ceil_half_x = (colors.shape()[0] + 1) / 2;
    let chars_shape = (ceil_half_x, colors.shape()[1]);
    let chars = Array2::from_elem(chars_shape, Some('▀'));
    for entity in &graph.entities {
        let pos = entity.get_position();
        colors[[pos.y, pos.x]] = Some(entity.color());
    }
    UnicodePixels::new(colors, chars)
}

impl Widget for GameWindow {
    #[allow(clippy::cast_precision_loss)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let graph = &self.world;
        let screen = render_graph(graph, area);

        for (yi, row) in screen.colors.outer_iter().enumerate() {
            for (xi, color) in row.iter().enumerate() {
                let curs = &mut buf[(xi as u16 + area.x, (yi / 2) as u16 + area.y)];
                if yi % 2 == 0 {
                    if let Some(color) = color {
                        curs.set_fg(*color);
                    }
                } else if let Some(color) = color {
                    curs.set_bg(*color);
                }
            }
        }

        for (yi, row) in screen.chars.outer_iter().enumerate() {
            for (xi, char) in row.iter().enumerate() {
                let curs = &mut buf[(xi as u16 + area.x, yi as u16 + area.y)];
                if let Some(char) = char {
                    curs.set_char(*char);
                }
            }
        }
    }
}
