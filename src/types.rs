use ratatui::prelude::Position;
use ratatui::style::Color;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
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
    pub coords: Coordinate,
}

#[derive(Debug, Clone)]
pub struct Chest {
    pub coords: Coordinate,
}

#[derive(Debug, Clone)]
pub enum Entity {
    Player(Player),
    Chest(Chest),
}

impl Entity {
    pub fn get_coords(&self) -> Coordinate {
        match self {
            Entity::Player(player) => player.coords.clone(),
            Entity::Chest(chest) => chest.coords.clone(),
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Entity::Player(_) => get_color(TolColor::VibMagenta),
            Entity::Chest(_) => get_color(TolColor::VibCyan),
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

impl Level {
    pub fn is_tile_occupied(&self, coord: &Coordinate) -> bool {
        match self.map[[coord.y, coord.x]] {
            Tile::Wall => true,
            _ => {
                for ent in self.entities.iter() {
                    if ent.get_coords() == *coord {
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

impl Widget for Level {
    #[allow(clippy::cast_precision_loss, clippy::similar_names)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut row_pixels;
        let mut row_iter = self.map.outer_iter();
        let mut yi: usize = 0;
        while let Some(row_top) = row_iter.next() {
            let maybe_row_bottom = row_iter.next();

            let top_colors;
            let bottom_colors;
            match maybe_row_bottom {
                Some(row_bottom) => {
                    top_colors = row_top.map(|ent| ent.color());
                    bottom_colors = row_bottom.map(|ent| ent.color());
                    row_pixels = top_colors.iter().zip(bottom_colors.iter());
                }
                None => {
                    top_colors = row_top.map(|ent| ent.color());
                    bottom_colors = row_top.map(|_| None);
                    row_pixels = top_colors.iter().zip(bottom_colors.iter());
                }
            }

            for (xi, (fg, bg)) in row_pixels.enumerate() {
                let curs = &mut buf[(xi as u16 + area.x, yi as u16 + area.y)];
                curs.set_char('▀');
                if let Some(fg) = fg {
                    curs.set_fg(*fg);
                }
                if let Some(bg) = bg {
                    curs.set_bg(*bg);
                }
            }

            yi += 1;
        }
        for entity in self.entities {
            let coords = entity.get_coords();

            let px_x = coords.x as u16 + area.x;
            let px_y = (coords.y / 2) as u16 + area.y;
            if area.contains(Position { x: px_x, y: px_y }) {
                let curs = &mut buf[(px_x, px_y)];
                if coords.y % 2 == 0 {
                    curs.set_fg(entity.color());
                } else {
                    curs.set_bg(entity.color());
                }
            }
        }
    }
}
