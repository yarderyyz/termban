use palette::{IntoColor, Okhsv, Srgb};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    widgets::{Block, Paragraph},
    Frame, Terminal,
};
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
use ratatui::{prelude::*, widgets::*};

use std::fs::File;
use std::io::{self, stdout, Read};
use ndarray::Array2;
use std::fmt;

static TILES: &str = " #@$.*+";

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

enum Action {
    Quit,
    Move(Direction),
    None,
}

#[derive(Debug)]
enum Token {
    Text(String),
    Entity(char),
    NewLine,
}

type Tokens = Vec<Token>;

#[derive(Debug)]
#[derive(Clone)]
enum Tile {
    Empty,
    Wall,
    Goal
}

#[derive(Debug)]
#[derive(Clone)]
struct Coordinates {
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
#[derive(Clone)]
struct Player {
    pub coords: Coordinates,
}

#[derive(Debug)]
#[derive(Clone)]
struct Chest {
    pub coords: Coordinates,
}

#[derive(Debug)]
#[derive(Clone)]
enum Entity {
    Player(Player),
    Chest(Chest)
}

#[derive(Debug, Clone)]
struct Level {
    pub name: String,
    pub map: Array2<Tile>,
    pub entities: Vec<Entity>
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
                },
                None => {
                    top_colors = row_top.map(|ent| ent.color());
                    bottom_colors = row_top.map(|_| None);
                    row_pixels = top_colors.iter().zip(bottom_colors.iter());
                }
            }

            for (xi, (fg, bg)) in row_pixels.enumerate() {
                let curs = &mut buf[(xi as u16 + area.x , yi as u16 + area.y)];
                curs.set_char('▀');
                if let Some(fg) = fg {
                    curs.set_fg(fg.clone());
                }
                if let Some(bg) = bg {
                    curs.set_bg(bg.clone());
                }
            }

            yi += 1;
        }
        for entity in self.entities {
            match entity {
                Entity::Player(player) => {
                    let px_x = player.coords.x;
                    let px_y = player.coords.y / 2;
                    let curs = &mut buf[(px_x as u16 + area.x , px_y as u16 + area.y)];
                    let color = Color::Rgb(0, 0, 255);
                    if px_y % 2 == 0 {
                        curs.set_fg(color);
                    } else {
                        curs.set_bg(color);
                    }
                }
                Entity::Chest(chest) => {
                    let px_x = chest.coords.x;
                    let px_y = chest.coords.y / 2;
                    let curs = &mut buf[(px_x as u16 + area.x , px_y as u16 + area.y)];
                    let color = Color::Rgb(255, 0, 255);
                    if px_y % 2 == 0 {
                        curs.set_fg(color);
                    } else {
                        curs.set_bg(color);
                    }
                }
            }
        }
    }
}

pub fn color_from_oklab(hue: f32, saturation: f32, value: f32) -> Color {
    let color: Srgb = Okhsv::new(hue, saturation, value).into_color();
    let color = color.into_format();
    Color::Rgb(color.red, color.green, color.blue)
}


impl Tile {
    fn color(&self) -> Option<Color> {
        match self {
            Tile::Wall => Some(Color::Rgb(65, 117, 0)),
            Tile::Goal => Some(Color::Rgb(230, 69, 0)),
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

fn get_board_dimensions(tokens: &[Token]) -> (usize, usize) {
    let mut ncols = 0;
    let nrows = tokens.iter()
        .filter(|tok| matches!(tok, Token::NewLine))
        .count();

    let mut count: usize = 0;
    for token in tokens {
        match token {
            Token::NewLine => {
                ncols = if count > ncols { count } else { ncols };
                count = 0;
            },
            _ => {
                count += 1;
            }
        }
    }

    return (ncols, nrows)
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn tokenize(contents: &str) -> Option<Tokens> {
    let mut tokens: Tokens = Tokens::new();
    for line in contents.lines() {
        if line.starts_with(';') {
            // TODO: Do this less dumb
            let mut line_chars = line.chars();
            line_chars.next();
            line_chars.next();
            tokens.push(Token::Text(line_chars.as_str().to_string()));
            continue;
        }
        if line.len() == 0 {
            continue;
        }
        for ch in line.chars() {
            if TILES.contains(ch) {
                tokens.push(Token::Entity(ch));
            } else {
                return None;
            }
        }
        tokens.push(Token::NewLine);
    }
    Some(tokens)
}

fn load_level(contents: &str) -> Result<Level, String> {
    let tokens = tokenize(contents);
    if tokens.is_none() {
        return Err("Level failed to load".to_string());
    }
    let tokens = tokens.unwrap();
    match tokens.as_slice() {
        [Token::Text(title), level_toks @ ..] => {
            // Dimensions for the board
            let (rows, cols)= get_board_dimensions(level_toks);

            // Create an initial board with default values (e.g., all `Wall`)
            let mut map = Array2::from_elem((cols, rows), Tile::Empty);
            let mut entities = Vec::new();

            let (mut col, mut row): (usize, usize) = (0,0);
            for tok in level_toks.iter() {
                match tok {
                    Token::Entity('#') => {
                        map[[row, col]] = Tile::Wall;
                    }
                    Token::Entity('@') => {
                        entities.push(Entity::Player(Player {
                            coords: Coordinates {
                                x: col, y: row
                            }
                        }));
                    }
                    Token::Entity('.') => {
                        map[[row, col]] = Tile::Goal;
                    }
                    Token::Entity('$') => {
                        entities.push(Entity::Chest(Chest {
                            coords: Coordinates {
                                x: col, y: row
                            }
                        }));
                    }
                    Token::Entity('*') => {
                        map[[row, col]] = Tile::Goal;
                        entities.push(Entity::Chest(Chest {
                            coords: Coordinates {
                                x: col, y: row
                            }
                        }));
                    }
                    Token::Entity('+') => {
                        map[[row, col]] = Tile::Goal;
                        entities.push(Entity::Player(Player {
                            coords: Coordinates {
                                x: col, y: row
                            }
                        }));
                    }
                    Token::NewLine => {
                        row += 1;
                        col = 0;
                        continue;
                    }
                    _ => {}
                }
                col += 1;
            }

            // Create an instance of Level
            let mut level = Level {
                name: title.to_string(),
                map,
                entities,
            };
            return Ok(level);
        },
        _ => {
            return Err("Level must start with a title".to_string());
        }
    }
}


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let filename = "./resources/levels/micro.ban";
    // TODO: actually handle errors here
    let mut level = read_file(filename)
        .map(|contents| load_level(&contents).unwrap())
        .unwrap();

    let title = level.name.clone();

    loop {
        terminal.draw(|frame: &mut Frame| {
            let area = frame.area();
            let outer_block = Block::bordered().title(title.clone());
            let inner = outer_block.inner(area);

            frame.render_widget(outer_block, area);
            frame.render_widget(level.clone(), inner);
        })?;
        match handle_events()? {
            Action::Quit => {
                break;
            },
            Action::Move(dir) => {
                // Iterate over mutable references to entities
                for entity in level.entities.iter_mut() {
                    if let Entity::Player(player) = entity {
                        match dir {
                            Direction::Up => player.coords.y -= 1,
                            Direction::Down => player.coords.y += 1,
                            Direction::Left => player.coords.x -= 1,
                            Direction:: Right => player.coords.x += 1,
                        }
                    }
                }
            }
            Action::None => {}
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<Action> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(Action::Quit);
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('w') {
                return Ok(Action::Move(Direction::Up));
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('s') {
                return Ok(Action::Move(Direction::Down));
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('a') {
                return Ok(Action::Move(Direction::Left));
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('d') {
                return Ok(Action::Move(Direction::Right));
            }
        }
    }
    Ok(Action::None)
}
