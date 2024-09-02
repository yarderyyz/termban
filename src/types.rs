use ratatui::style::Color;

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
    ZoomClose,
    ZoomMiddle,
    ZoomFar,
}

#[derive(Debug, Clone)]
pub enum Zoom {
    Close,
    Middle,
    Far,
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
impl Coordinate {
    pub fn arr_index(&self) -> [usize; 2] {
        [self.y, self.x]
    }
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

    // If every soko_box entity share a coordinate space with a goal tile, that means you win!
    pub fn is_sokoban_solved(&self) -> bool {
        self.entities
            .iter()
            .filter(|ent| matches!(ent, Entity::SokoBox(_)))
            .all(|ent| {
                if let Entity::SokoBox(soko_box) = ent {
                    let tile = &self.map[[soko_box.coords.y, soko_box.coords.x]];
                    matches!(tile, Tile::Goal)
                } else {
                    false // This line should never be reached due to the filter
                }
            })
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
    pub zoom: Zoom,
}

#[derive(Debug, Clone)]
pub struct GlyphCell {
    pub glyph: char,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
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

pub type GlyphCells = Array2<GlyphCell>;

#[derive(Debug, Clone)]
pub enum RenderItem {
    Board(Board),
    Entity(Entity),
}

#[derive(Debug, Clone)]
pub struct RenderNode {
    pub item: RenderItem,
    pub children: Option<Vec<RenderNode>>,
}

#[derive(Debug, Clone)]
pub struct RenderGraph {
    pub root: RenderNode,
}
