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
    fn maybe_move(
        &mut self,
    );
}

#[derive(
    Debug, Clone,
)]
pub enum Tile {
    Empty,
    Wall,
    Goal,
}

#[derive(
    Debug, Clone,
)]
pub struct Coordinates
{
    pub x: usize,
    pub y: usize,
}

#[derive(
    Debug, Clone,
)]
pub struct Player {
    pub coords:
        Coordinates,
    pub color:
        Color,
}

#[derive(
    Debug, Clone,
)]
pub struct Chest {
    pub coords:
        Coordinates,
    pub color:
        Color,
}

impl Movable
    for Player
{
    fn maybe_move(
        &mut self,
    ) {
        // Implement the logic for moving a player
    }
}

impl Movable
    for Chest
{
    fn maybe_move(
        &mut self,
    ) {
        // Implement the logic for moving a chest
    }
}

#[derive(
    Debug, Clone,
)]
pub enum Entity {
    Player(Player),
    Chest(Chest),
}

impl Entity {
    pub fn maybe_move(
        &mut self,
    ) {
        match self {
            Entity::Player(player) => player.maybe_move(),
            Entity::Chest(chest) => chest.maybe_move(),
        }
    }
}

#[derive(
    Debug, Clone,
)]
pub struct Level {
    pub name:
        String,
    pub map: Array2<
        Tile,
    >,
    pub entities:
        Vec<Entity>,
}

impl Tile {
    pub fn color(
        &self,
    ) -> Option<Color>
    {
        match self {
            Tile::Wall => Some(get_color(TolColor::VibTeal)),
            Tile::Goal => Some(get_color(TolColor::VibRed)),
            Tile::Empty => Some(Color::Rgb(41, 19, 10)),
        }
    }
}

impl fmt::Display
    for Tile
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Empty => write!(f, " "),
            Tile::Goal => write!(f, "."),
        }
    }
}

impl fmt::Display
    for Level
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        writeln!(
            f
        )?;
        for row in self.map.rows() {
            for entity in row {
                write!(f, "{}", entity)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
