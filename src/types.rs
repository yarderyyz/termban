use ndarray::prelude::arr2;
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
pub struct Ball {
    pub coords: Coordinate,
}

#[derive(Debug, Clone)]
pub enum Entity {
    Player(Player),
    Ball(Ball),
}

impl Entity {
    pub fn get_coords(&self) -> Coordinate {
        match self {
            Entity::Player(player) => player.coords.clone(),
            Entity::Ball(ball) => ball.coords.clone(),
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Entity::Player(_) => Color::Rgb(255, 0, 0),
            Entity::Ball(_) => get_color(TolColor::BriBlue),
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
            Tile::Wall => Some(get_color(TolColor::LigLightBlue)),
            Tile::Goal => Some(get_color(TolColor::BriGrey)),
            Tile::Empty => Some(Color::Rgb(222, 255, 255)),
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

#[derive(Debug, Clone)]
struct CharPixel {
    char: char,
    fg: Color,
    bg: Color,
}

#[derive(Debug, Clone)]
struct Sprite {
    chars: Array2<CharPixel>,
}

const BOX_ELEMENTS: [char; 32] = [
    '▀', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█', '▉', '▊', '▋', '▌', '▍', '▎', '▏',
    '▐', '░', '▒', '▓', '▔', '▕', '▖', '▗', '▘', '▙', '▚', '▛', '▜', '▝', '▞', '▟',
];

impl Level {
    fn get_player_sprite_8_simple() -> Sprite {
        let background = Color::Rgb(133, 133, 133);
        let hair1 = Color::Rgb(52, 32, 33);
        let hair2 = Color::Rgb(72, 42, 43);
        let skin = Color::Rgb(239, 204, 165);
        let eye = Color::Rgb(19, 19, 19);
        let cloth1 = Color::Rgb(8, 25, 61);
        let cloth2 = Color::Rgb(37, 75, 75);
        let cloth3 = Color::Rgb(56, 109, 82);
        #[rustfmt::skip]
        #[allow(clippy::cast_precision_loss)]
        let chars = arr2(
            &[
                [CharPixel {char: '▀', fg: Color::from_u32(0),      bg: background},
                 CharPixel {char: '▀', fg: background,      bg: hair1},
                 CharPixel {char: '▀', fg: hair1,           bg: hair2},
                 CharPixel {char: '▀', fg: hair1,           bg: hair2},
                 CharPixel {char: '▀', fg: hair1,           bg: hair2},
                 CharPixel {char: '▀', fg: hair2,           bg: hair1},
                 CharPixel {char: '▀', fg: background,      bg: hair1},
                 CharPixel {char: '▀', fg: background,      bg: background},
                ],
                [CharPixel {char: '▀', fg: background,      bg: background},
                 CharPixel {char: '▀', fg: hair1,           bg: background},
                 CharPixel {char: '▀', fg: eye,             bg: skin},
                 CharPixel {char: '▀', fg: skin,            bg: skin},
                 CharPixel {char: '▀', fg: eye,             bg: skin},
                 CharPixel {char: '▀', fg: hair1,           bg: skin},
                 CharPixel {char: '▀', fg: hair1,           bg: background},
                 CharPixel {char: '▀', fg: Color::from_u32(0),      bg: background},
                ],
                [CharPixel {char: '▀', fg: background,      bg: background},
                 CharPixel {char: '▀', fg: background,      bg: cloth1},
                 CharPixel {char: '▀', fg: cloth2,          bg: cloth3},
                 CharPixel {char: '▀', fg: cloth2,          bg: cloth2},
                 CharPixel {char: '▀', fg: cloth2,          bg: cloth2},
                 CharPixel {char: '▀', fg: background,      bg: cloth2},
                 CharPixel {char: '▀', fg: background,      bg: background},
                 CharPixel {char: '▀', fg: background,      bg: background},
                ],
                [CharPixel {char: '▀', fg: background,      bg: background},
                 CharPixel {char: '▀', fg: cloth1,          bg: background},
                 CharPixel {char: '▀', fg: cloth1,          bg: cloth1},
                 CharPixel {char: '▀', fg: cloth3,          bg: cloth3},
                 CharPixel {char: '▀', fg: cloth2,          bg: cloth3},
                 CharPixel {char: '▀', fg: cloth2,          bg: cloth3},
                 CharPixel {char: '▀', fg: cloth2,          bg: cloth3},
                 CharPixel {char: '▀', fg: background,      bg: background},
                ],

            ]
        );
        Sprite { chars }
    }
    fn get_player_sprite_8() -> Sprite {
        let background = Color::Rgb(133, 133, 133);
        let hair1 = Color::Rgb(52, 32, 33);
        let hair2 = Color::Rgb(72, 42, 43);
        let skin = Color::Rgb(239, 204, 165);
        let eye = Color::Rgb(19, 19, 19);
        let cloth1 = Color::Rgb(8, 25, 61);
        let cloth2 = Color::Rgb(37, 75, 75);
        let cloth3 = Color::Rgb(56, 109, 82);
        let cloth4 = Color::Rgb(45, 95, 65);
        let flower = Color::Rgb(255, 192, 203);
        #[rustfmt::skip]
        #[allow(clippy::cast_precision_loss)]
        let chars = arr2(
            &[
                [CharPixel {char: '@', fg: Color::from_u32(0),      bg: background},
                 CharPixel {char: '▅', fg: background,      bg: hair1},
                 CharPixel {char: '▚', fg: hair1,           bg: hair2},
                 CharPixel {char: '▚', fg: hair1,           bg: hair2},
                 CharPixel {char: '▚', fg: hair1,           bg: hair2},
                 CharPixel {char: '▝', fg: hair2,           bg: hair1},
                 CharPixel {char: '▀', fg: background,      bg: hair1},
                 CharPixel {char: '▀', fg: background,      bg: background},
                ],
// (◠)
                [CharPixel {char: '▀', fg: background,      bg: background},
                 CharPixel {char: '▂', fg: background,      bg: hair1},
                 CharPixel {char: '◠', fg: eye,             bg: skin},
                 CharPixel {char: '‿', fg: eye,            bg: skin},
                 CharPixel {char: '◠', fg: eye,             bg: skin},
                 CharPixel {char: '✿', fg: flower,            bg: hair1},
                 CharPixel {char: '▀', fg: hair1,           bg: background},
                 CharPixel {char: '@', fg: Color::from_u32(0),      bg: background},
                ],
                [CharPixel {char: '▀', fg: background,      bg: background},
                 CharPixel {char: '▀', fg: background,      bg: cloth1},
                 CharPixel {char: '▚', fg: cloth3,          bg: cloth2},
                 CharPixel {char: '▀', fg: cloth2,          bg: cloth2},
                 CharPixel {char: '▚', fg: cloth2,          bg: cloth4},
                 CharPixel {char: '▖', fg: cloth2,          bg: background},
                 CharPixel {char: '▀', fg: background,      bg: background},
                 CharPixel {char: '▀', fg: background,      bg: background},
                ],
                [CharPixel {char: '▀', fg: background,      bg: background},
                 CharPixel {char: '▝', fg: cloth1,          bg: background},
                 CharPixel {char: '▙', fg: cloth1,          bg: cloth2},
                 CharPixel {char: '▙', fg: cloth3,          bg: cloth2},
                 CharPixel {char: '▀', fg: cloth2,          bg: cloth3},
                 CharPixel {char: '▂', fg: cloth3,          bg: cloth2},
                 CharPixel {char: '▂', fg: cloth3,          bg: background},
                 CharPixel {char: '▀', fg: background,      bg: background},
                ],

            ]
        );
        Sprite { chars }
    }

    fn get_player_sprite_6() -> Sprite {
        let background = Color::Rgb(133, 133, 133);
        let hair1 = Color::Rgb(52, 32, 33);
        let hair2 = Color::Rgb(72, 42, 43);
        let skin = Color::Rgb(239, 204, 165);
        let cloth1 = Color::Rgb(8, 25, 61);
        let cloth2 = Color::Rgb(37, 75, 75);
        let cloth3 = Color::Rgb(56, 109, 82);
        #[rustfmt::skip]
        #[allow(clippy::cast_precision_loss)]
        let chars = arr2(
            &[
                [CharPixel {char: '@', fg: Color::from_u32(0),      bg: background},
                 CharPixel {char: '▅', fg: background,      bg: hair1},
                 CharPixel {char: '▚', fg: hair1,           bg: hair2},
                 CharPixel {char: '▝', fg: hair2,           bg: hair1},
                 CharPixel {char: '▀', fg: background,      bg: hair1},
                 CharPixel {char: '▀', fg: background,      bg: background},
                ],
                [CharPixel {char: '▀', fg: background,      bg: background},
                 CharPixel {char: '▂', fg: background,      bg: hair1},
                 CharPixel {char: '▆', fg: skin,            bg: hair1},
                 CharPixel {char: '▆', fg: skin,            bg: hair1},
                 CharPixel {char: '▂', fg: skin,            bg: hair1},
                 CharPixel {char: '▀', fg: hair1,           bg: background},
                ],
                [CharPixel {char: '▀', fg: background,      bg: background},
                 CharPixel {char: '▝', fg: cloth1,          bg: background},
                 CharPixel {char: '▚', fg: cloth3,          bg: cloth2},
                 CharPixel {char: '▀', fg: cloth2,          bg: cloth2},
                 CharPixel {char: '▙', fg: cloth2,          bg: background},
                 CharPixel {char: '▖', fg: cloth2,      bg: background},
                ],
            ]
        );
        Sprite { chars }
    }

    fn get_player_sprite_4() -> Sprite {
        let background = Color::Rgb(133, 133, 133);
        let hair1 = Color::Rgb(52, 32, 33);
        let skin = Color::Rgb(239, 204, 165);
        let cloth1 = Color::Rgb(8, 25, 61);
        let cloth2 = Color::Rgb(37, 75, 75);
        let cloth3 = Color::Rgb(56, 109, 82);
        #[rustfmt::skip]
        #[allow(clippy::cast_precision_loss)]
        let chars = arr2(
            &[
                [CharPixel {char: '▝', fg: hair1,           bg: background},
                 CharPixel {char: '▅', fg: skin,            bg: hair1},
                 CharPixel {char: '▂', fg: skin,            bg: hair1},
                 CharPixel {char: '▙', fg: hair1,           bg: background},
                ],
                [CharPixel {char: '▝', fg: cloth1,          bg: background},
                 CharPixel {char: '▚', fg: cloth3,          bg: cloth2},
                 CharPixel {char: '▀', fg: cloth2,          bg: cloth2},
                 CharPixel {char: '▖', fg: cloth2,          bg: background},
                ],
            ]
        );
        Sprite { chars }
    }
}

impl Widget for Level {
    #[allow(clippy::cast_precision_loss)]
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

        for (index, elem) in BOX_ELEMENTS.iter().enumerate() {
            let curs = &mut buf[(index as u16 + area.x, (yi + 1) as u16 + area.y)];
            curs.set_char(*elem);
            curs.set_fg(get_color(TolColor::VibRed));
            curs.set_bg(get_color(TolColor::VibBlue));
        }

        let start_row = yi + 2;
        let start_column = 0;
        let sprite = Level::get_player_sprite_8_simple();
        for (yi, row) in sprite.chars.outer_iter().enumerate() {
            for (xi, char_px) in row.iter().enumerate() {
                let curs = &mut buf[(
                    (xi + start_column) as u16 + area.x,
                    (yi + start_row) as u16 + area.y,
                )];
                curs.set_char(char_px.char);
                curs.set_fg(char_px.fg);
                curs.set_bg(char_px.bg);
            }
        }

        let start_row = yi + 2;
        let start_column = 9;
        let sprite = Level::get_player_sprite_8();
        for (yi, row) in sprite.chars.outer_iter().enumerate() {
            for (xi, char_px) in row.iter().enumerate() {
                let curs = &mut buf[(
                    (xi + start_column) as u16 + area.x,
                    (yi + start_row) as u16 + area.y,
                )];
                curs.set_char(char_px.char);
                curs.set_fg(char_px.fg);
                curs.set_bg(char_px.bg);
            }
        }

        let start_row = yi + 3;
        let start_column = 18;
        let sprite = Level::get_player_sprite_6();
        for (yi, row) in sprite.chars.outer_iter().enumerate() {
            for (xi, char_px) in row.iter().enumerate() {
                let curs = &mut buf[(
                    (xi + start_column) as u16 + area.x,
                    (yi + start_row) as u16 + area.y,
                )];
                curs.set_char(char_px.char);
                curs.set_fg(char_px.fg);
                curs.set_bg(char_px.bg);
            }
        }

        let start_row = yi + 4;
        let start_column = 25;
        let sprite = Level::get_player_sprite_4();
        for (yi, row) in sprite.chars.outer_iter().enumerate() {
            for (xi, char_px) in row.iter().enumerate() {
                let curs = &mut buf[(
                    (xi + start_column) as u16 + area.x,
                    (yi + start_row) as u16 + area.y,
                )];
                curs.set_char(char_px.char);
                curs.set_fg(char_px.fg);
                curs.set_bg(char_px.bg);
            }
        }

        let start_row = yi + 1;
        let start_column = 30;
        let curs = &mut buf[(
            (start_column) as u16 + area.x,
            (yi + start_row) as u16 + area.y,
        )];
        curs.set_char('▀');
        let hair1 = Color::Rgb(52, 32, 33);
        let cloth2 = Color::Rgb(37, 75, 75);
        curs.set_fg(hair1);
        curs.set_bg(cloth2);
    }
}
