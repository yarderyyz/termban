use ratatui::prelude::*;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

use std::fs::File;
use std::io::{self, stdout, Read};

mod colors;
mod soko_loader;
mod types;

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let filename = "./resources/levels/micro.ban";
    // TODO: actually handle errors here
    let mut level = read_file(filename)
        .map(|contents| soko_loader::load_level(&contents).unwrap())
        .unwrap();

    let title = level.name.clone();

    loop {
        let mut debug: Vec<String> = Vec::new();

        for entity in level.entities.iter_mut() {
            if let types::Entity::Player(player) = entity {
                debug.push(format!("{:?}", player.coords.clone()));
            }
        }

        terminal.draw(|frame: &mut Frame| {
            let main_area = frame.area();

            let [left_area, right_area] = Layout::horizontal([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .areas(main_area);

            let outer_left_block = Block::bordered().title(title.clone());
            let inner_left = outer_left_block.inner(left_area);

            frame.render_widget(outer_left_block, left_area);
            frame.render_widget(level.clone(), inner_left);

            let text = debug.join("\n");
            frame.render_widget(
                Paragraph::new(text).block(Block::bordered().title("debug")),
                right_area,
            );
        })?;

        match handle_events()? {
            types::Action::Quit => {
                break;
            }
            types::Action::Move(direction) => {
                let mut player_move = None;
                // First we find the player and figure out what its new coords will be.
                // if the player is trying to move into a wall we'll do nothing otherwise we'll
                // set the move
                for (index, entity) in level.entities.iter().enumerate() {
                    if let types::Entity::Player(player) = entity {
                        let new_chords =
                            get_new_coords(player.coords.clone(), &direction);

                        match level.map[[new_chords.y, new_chords.x]] {
                            types::Tile::Wall => player_move = None,
                            _ => player_move = Some((index, new_chords)),
                        }
                        break;
                    }
                }

                let mut chest_move = None;
                if let Some((_, player_coords)) = player_move.clone() {
                    for (index, entity) in level.entities.iter().enumerate() {
                        if let types::Entity::Chest(chest) = entity {
                            // if there is a chest where the player wants to move see if we can
                            // push it.
                            if chest.coords == player_coords.clone() {
                                let new_coord =
                                    get_new_coords(chest.coords.clone(), &direction);

                                if level.is_tile_occupied(&new_coord) {
                                    // if the tile we are trying to move too is occupied both moves are
                                    // invalid.
                                    chest_move = None;
                                    player_move = None;
                                } else {
                                    // otherwise move the chest
                                    chest_move = Some((index, new_coord.clone()));
                                }
                            }
                        }
                    }
                }

                // resolve the movement
                if let Some((index, new_coords)) = player_move {
                    if let types::Entity::Player(ref mut player) =
                        &mut level.entities[index]
                    {
                        player.coords = new_coords.clone();
                    }
                }
                if let Some((index, new_coords)) = chest_move {
                    if let types::Entity::Chest(ref mut chest) =
                        &mut level.entities[index]
                    {
                        chest.coords = new_coords.clone();
                    }
                }
            }
            types::Action::None => {}
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn get_new_coords(
    coords: types::Coordinate,
    direction: &types::Direction,
) -> types::Coordinate {
    match direction {
        types::Direction::Up => types::Coordinate {
            x: coords.x,
            y: if coords.y > 0 { coords.y - 1 } else { 0 },
        },
        types::Direction::Down => types::Coordinate {
            x: coords.x,
            y: coords.y + 1,
        },
        types::Direction::Left => types::Coordinate {
            x: if coords.x > 0 { coords.x - 1 } else { 0 },
            y: coords.y,
        },
        types::Direction::Right => types::Coordinate {
            x: coords.x + 1,
            y: coords.y,
        },
    }
}

fn handle_events() -> io::Result<types::Action> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return process_key_press(key.code);
            }
        }
    }
    Ok(types::Action::None)
}

fn process_key_press(key_code: KeyCode) -> io::Result<types::Action> {
    match key_code {
        KeyCode::Char('q') => Ok(types::Action::Quit),
        KeyCode::Char('w') => Ok(types::Action::Move(types::Direction::Up)),
        KeyCode::Char('s') => Ok(types::Action::Move(types::Direction::Down)),
        KeyCode::Char('a') => Ok(types::Action::Move(types::Direction::Left)),
        KeyCode::Char('d') => Ok(types::Action::Move(types::Direction::Right)),
        _ => Ok(types::Action::None),
    }
}
