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

#[derive(Debug)]
enum Token {
    Text(String),
    Entity(char),
    NewLine
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
    x: usize,
    y: usize
}

#[derive(Debug)]
#[derive(Clone)]
struct Player {
    coords: Coordinates,
}

#[derive(Debug)]
#[derive(Clone)]
struct Box {
    coords: Coordinates,
}

#[derive(Debug)]
#[derive(Clone)]
enum Entity {
    Player(Player),
    Box(Box)
}

#[derive(Debug)]
struct Level {
    name: String,
    map: Array2<Tile>,
    entities: Vec<Entity>
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
                Entity::Box(b) => {
                    let px_x = b.coords.x;
                    let px_y = b.coords.y / 2;
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

fn load_level(contents: &str) -> Option<Level> {
    let tokens = tokenize(contents);
    if tokens.is_none() {
        println!("Level failed to load");
        return None;
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
                        entities.push(Entity::Box(Box {
                            coords: Coordinates {
                                x: col, y: row
                            }
                        }));
                    }
                    Token::Entity('*') => {
                        map[[row, col]] = Tile::Goal;
                        entities.push(Entity::Box(Box {
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
            let level = Level {
                name: title.to_string(),
                map,
                entities,
            };
            return Some(level);
        },
        _ => {
            println!("Level must start with a title");
            return None;
        }
    }
}



fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    let mut maybe_level = None;
    let filename = "./resources/levels/micro.ban";
    match read_file(filename) {
        Ok(contents) => {
            maybe_level = load_level(&contents);
            ()
        },
        Err(e) => println!("Error reading file: {}", e),
    }

    let title = match &maybe_level {
        Some(level) => {
            level.name.clone()

        },
        None => {"Default".to_string()}
    };

    match maybe_level {
        Some(level) => {
            let area = frame.area();
            let outer_block = Block::bordered().title(title);
            let inner = outer_block.inner(area);

            frame.render_widget(outer_block, area);
            frame.render_widget(level, inner);
        },
        None => ()
    };
}
