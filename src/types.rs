use ratatui::style::Color;
use std::fmt;

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
    None,
}

pub trait Movable {
    fn maybe_move(&mut self, direction: Direction, board: Board);
}

#[derive(Debug, Clone)]
pub enum Tile {
    Empty,
    Wall,
    Goal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub coords: Coordinates,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Chest {
    pub coords: Coordinates,
    pub color: Color,
}

impl Movable for Player {
    fn maybe_move(&mut self, direction: Direction, board: Board) {
        // Implement the logic for moving a player
    }
}

impl Movable for Chest {
    fn maybe_move(&mut self, direction: Direction, board: Board) {
        // Implement the logic for moving a chest
    }
}

#[derive(Debug, Clone)]
pub enum Entity {
    Player(Player),
    Chest(Chest),
}

impl Entity {
    pub fn maybe_move(&mut self, direction: Direction, board: Board) {
        match self {
            Entity::Player(player) => player.maybe_move(direction, board),
            Entity::Chest(chest) => chest.maybe_move(direction, board),
        }
    }
    pub fn get_coords(&self) -> Coordinates {
        match self {
            Entity::Player(player) => player.coords.clone(),
            Entity::Chest(chest) => chest.coords.clone(),
        }
    }
}

type Board = Array2<Tile>;

#[derive(Debug, Clone)]
pub struct Level {
    pub name: String,
    pub map: Board,
    pub entities: Vec<Entity>,
}

impl Tile {
    pub fn color(&self) -> Option<Color> {
        match self {
            Tile::Wall => Some(get_color(TolColor::VibTeal)),
            Tile::Goal => Some(get_color(TolColor::VibRed)),
            Tile::Empty => Some(Color::Rgb(41, 19, 10)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Empty => write!(f, " "),
            Tile::Goal => write!(f, "."),
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for row in self.map.rows() {
            for entity in row {
                write!(f, "{}", entity)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
