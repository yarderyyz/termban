use std::fmt;
use ratatui::style::Color;

use ndarray::Array2;
use crate::colors::{TolColor, get_color_map};

#[derive(Debug)]
#[derive(Clone)]
pub enum Tile {
    Empty,
    Wall,
    Goal
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Player {
    pub coords: Coordinates,
    pub color: Color,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Chest {
    pub coords: Coordinates,
    pub color: Color,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Entity {
    Player(Player),
    Chest(Chest)
}

#[derive(Debug, Clone)]
pub struct Level {
    pub name: String,
    pub map: Array2<Tile>,
    pub entities: Vec<Entity>
}


impl Tile {
    pub fn color(&self) -> Option<Color> {
        let color_map = get_color_map();
        match self {
            Tile::Wall => Some(color_map.get(&TolColor::VibOrange).unwrap().clone()),
            Tile::Goal => Some(color_map.get(&TolColor::VibRed).unwrap().clone()),
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
