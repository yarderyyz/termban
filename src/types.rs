use ratatui::style::Color;

use crate::colors::{get_color, TolColor};
use ndarray::Array2;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveFile {
    pub saves: Vec<Save>,
}
impl SaveFile {
    pub fn new() -> Self {
        Self {
            saves: vec![Save {
                name: "My Save".to_string(),
                level: 0,
            }],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Save {
    pub name: String,
    pub level: usize,
}

#[derive(Debug)]
pub struct Model {
    pub running_state: RunningState,
    pub soko_game: SokoModel,
    pub worlds: Vec<World>,
    pub world_index: usize,
    pub save_file: SaveFile,
}

impl Model {
    pub fn change_level(self: &mut Model, level_index: usize) {
        self.world_index = level_index;
        let world = self.worlds[self.world_index].clone();
        self.soko_game = SokoModel::from(world)
    }

    pub fn increment_level(self: &mut Model) {
        if self.world_index != self.worlds.len() - 1 {
            self.change_level(self.world_index + 1);
        }
    }

    pub fn decrement_level(self: &mut Model) {
        if self.world_index != 0 {
            self.change_level(self.world_index - 1);
        }
    }

    /// Erasing your history, erases your past
    pub fn erase_history(self: &mut Model) {
        self.soko_game.history.clear();
    }
    /// Let's start over from the beginning.
    pub fn refresh_window(self: &mut Model) {
        if let Some(prev_world_state) = self.soko_game.history.first() {
            self.soko_game.current_state = prev_world_state.clone();
        }
        self.erase_history();
    }
}

#[derive(Debug, PartialEq, Eq)]
// XXX: Ambiguous name?
pub enum RunningState {
    Menu,
    LevelSelect,
    Game,
    Done,
}

#[derive(PartialEq)]
pub enum MenuAction {
    StartGame,
    Quit,
    EraseSaveData,
}

// #[derive(PartialEq)]
pub enum LevelSelectAction {
    Up,
    Down,
    Select,
    // PageUp,
    // PageDown,
    Quit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameAction {
    None,
    Quit,
    Move(Direction),
    Undo,
    Reset,
    ZoomClose,
    ZoomMiddle,
    ZoomFar,
    Win,
}

#[derive(Debug, Clone)]
pub enum Zoom {
    Close,
    Middle,
    Far,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Floor,
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
pub enum Entity {
    Player { position: Coordinate },
    SokoBox { position: Coordinate },
}

impl Entity {
    pub fn get_position(&self) -> Coordinate {
        match self {
            Entity::Player { position } => position.clone(),
            Entity::SokoBox { position } => position.clone(),
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Entity::Player { .. } => get_color(TolColor::PurRed),
            Entity::SokoBox { .. } => get_color(TolColor::BriBlue),
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
                for entity in self.entities.iter() {
                    if entity.get_position() == *coord {
                        return true;
                    }
                }
                false
            }
        }
    }

    // If every soko_box entity share a coordinate space with a goal tile, that means you win!
    pub fn is_sokoban_solved(&self) -> bool {
        self.entities.iter().all(|entity| {
            if let Entity::SokoBox { position } = entity {
                let tile = &self.board[[position.y, position.x]];
                matches!(tile, Tile::Goal)
            } else {
                // Other entities don't count towards the win condition so return true
                true
            }
        })
    }
}

impl Tile {
    pub fn color(&self) -> Option<Color> {
        match self {
            Tile::Wall => Some(get_color(TolColor::LigLightBlue)),
            Tile::Goal => Some(get_color(TolColor::BriGrey)),
            Tile::Floor => Some(get_color(TolColor::CstLigBlue)),
            Tile::Empty => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SokoModel {
    pub current_state: World,
    pub zoom: Zoom,
    pub history: Vec<World>,
    pub debug: Vec<String>,
}

impl SokoModel {
    pub fn from(world: World) -> Self {
        Self {
            current_state: world,
            zoom: Zoom::Middle,
            history: Vec::new(),
            debug: Vec::new(),
        }
    }
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

impl Default for GlyphCell {
    fn default() -> Self {
        GlyphCell::new()
    }
}

pub type GlyphCells = Array2<GlyphCell>;

#[derive(Debug, Clone)]
pub enum RenderItem<'a> {
    Board(&'a Board),
    Entity(&'a Entity),
}

#[derive(Debug, Clone)]
pub struct RenderNode<'a> {
    pub item: RenderItem<'a>,
    pub children: Option<Vec<RenderNode<'a>>>,
}

#[derive(Debug, Clone)]
pub struct RenderGraph<'a> {
    pub root: RenderNode<'a>,
}
