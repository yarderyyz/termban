/*!
 * Author: Lee Gauthier
 *
 * Description:
 * This module is responsible for loading Sokoban game levels designed by David W. Skinner.
 * It provides functionality to parse and load level data from files into the game structure.
 * Each level is parsed and constructed into a game-ready format to be utilized by the game engine.
 *
 * This loader supports the standard Sokoban level format as specified by David W. Skinner,
 * ensuring compatibility and correctness across different Sokoban level sets.
 *
 * Usage:
 * The module functions are intended to be used by the game engine to initialize and start
 * new games with pre-defined levels. Errors in file format or content are handled gracefully
 * and reported to the caller.
 *
 * Notes:
 * The levels are expected to be stored in plain text format with each level separated by
 * a newline and starting with a level identifier.
 */

use crate::colors::{get_color, TolColor};
use crate::types::{Chest, Coordinate, Entity, Level, Player, Tile};
use ndarray::Array2;

static TILES: &str = " #@$.*+";

#[derive(Debug)]
enum Token {
    Text(String),
    Entity(char),
    NewLine,
}

type Tokens = Vec<Token>;

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
        if line.is_empty() {
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

fn get_board_dimensions(tokens: &[Token]) -> (usize, usize) {
    let mut ncols = 0;
    let nrows = tokens
        .iter()
        .filter(|tok| matches!(tok, Token::NewLine))
        .count();

    let mut count: usize = 0;
    for token in tokens {
        match token {
            Token::NewLine => {
                ncols = if count > ncols { count } else { ncols };
                count = 0;
            }
            _ => {
                count += 1;
            }
        }
    }
    (ncols, nrows)
}

pub fn load_level(contents: &str) -> Result<Level, String> {
    let chest_color = get_color(TolColor::VibMagenta);
    let player_color = get_color(TolColor::VibCyan);

    let tokens = tokenize(contents);
    if tokens.is_none() {
        return Err("Level failed to load".to_string());
    }
    let tokens = tokens.unwrap();
    match tokens.as_slice() {
        [Token::Text(title), level_toks @ ..] => {
            // Dimensions for the board
            let (rows, cols) = get_board_dimensions(level_toks);

            // Create an initial board with default values (e.g., all `Wall`)
            let mut map = Array2::from_elem((cols, rows), Tile::Empty);
            let mut entities = Vec::new();

            let (mut col, mut row): (usize, usize) = (0, 0);
            for tok in level_toks.iter() {
                match tok {
                    Token::Entity('#') => {
                        map[[row, col]] = Tile::Wall;
                    }
                    Token::Entity('@') => {
                        entities.push(Entity::Player(Player {
                            coords: Coordinate { x: col, y: row },
                            color: player_color,
                        }));
                    }
                    Token::Entity('.') => {
                        map[[row, col]] = Tile::Goal;
                    }
                    Token::Entity('$') => {
                        entities.push(Entity::Chest(Chest {
                            coords: Coordinate { x: col, y: row },
                            color: chest_color,
                        }));
                    }
                    Token::Entity('*') => {
                        map[[row, col]] = Tile::Goal;
                        entities.push(Entity::Chest(Chest {
                            coords: Coordinate { x: col, y: row },
                            color: chest_color,
                        }));
                    }
                    Token::Entity('+') => {
                        map[[row, col]] = Tile::Goal;
                        entities.push(Entity::Player(Player {
                            coords: Coordinate { x: col, y: row },
                            color: player_color,
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
            Ok(level)
        }
        _ => Err("Level must start with a title".to_string()),
    }
}
