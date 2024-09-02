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
mod render;
mod soko_loader;
mod sprites;
mod types;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
enum RunningState {
    #[default]
    Menu,
    Game,
}

#[derive(Debug, Default, Clone)]
struct AppModel {
    state: RunningState,
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn play_game<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut history = Vec::new();

    let ban_filename = "./resources/levels/micro2.ban";
    // TODO: actually handle errors here
    //
    let worlds = read_file(ban_filename)
        .map(|contents| soko_loader::parse_sokoban_worlds(&contents).unwrap())
        .unwrap();

    let starting_world = 33;
    let title = worlds[starting_world].name.clone();

    let mut game_window = types::GameWindow {
        world: worlds[starting_world].clone(),
        zoom: types::Zoom::Far,
    };

    loop {
        let mut debug: Vec<String> = Vec::new();

        for entity in game_window.world.entities.iter() {
            if let types::Entity::Player(player) = entity {
                debug.push(format!("{:?}", player.position.clone()));
            }
        }
        debug.push(format!("{:?}", &game_window.world.board.dim()));

        if game_window.world.is_sokoban_solved() {
            debug.push("You win!".to_string());
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
            frame.render_widget(game_window.clone(), inner_left);

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
                if let Some(new_level) = handle_move(&game_window.world, direction) {
                    history.push(game_window.world.clone());
                    game_window.world = new_level;
                }
            }
            types::Action::Undo => {
                if let Some(prev_level) = history.pop() {
                    game_window.world = prev_level;
                }
            }
            types::Action::Reset => {
                history.push(game_window.world.clone());
                if let Some(prev_level) = history.first() {
                    game_window.world = prev_level.clone();
                }
            }
            types::Action::ZoomClose => game_window.zoom = types::Zoom::Close,
            types::Action::ZoomMiddle => game_window.zoom = types::Zoom::Middle,
            types::Action::ZoomFar => game_window.zoom = types::Zoom::Far,
            types::Action::None => {}
        };
    }
    Ok(())
}

fn start_menu<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        let mut debug: Vec<String> = Vec::new();

        terminal.draw(|frame: &mut Frame| {
            debug.push("Debug".to_string());
            let main_area = frame.area();

            let [left_area, right_area] = Layout::horizontal([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .areas(main_area);

            let outer_left_block = Block::bordered().title("SOKOBAN");
            // let inner_left = outer_left_block.inner(left_area);

            frame.render_widget(outer_left_block, left_area);

            let text = debug.join("\n");
            frame.render_widget(
                Paragraph::new(text).block(Block::bordered().title("debug")),
                right_area,
            );
        })?;

        if let types::Action::Quit = handle_events()? {
            break;
        }
        //match handle_events()? {
        //    types::Action::Quit => {
        //        break;
        //    }
        //    _ => {}
        //};
    }
    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut action = types::MenuAction::None;
    loop {
        let app_model = process_app_state(action.clone());
        match app_model.state {
            RunningState::Menu => {
                start_menu(&mut terminal)?;
                action = types::MenuAction::StartGame;
            }
            RunningState::Game => {
                play_game(&mut terminal)?;
                break;
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn process_app_state(action: types::MenuAction) -> AppModel {
    match action {
        types::MenuAction::None => AppModel::default(),
        types::MenuAction::StartGame => AppModel {
            state: RunningState::Game,
        },
    }
}

fn handle_move(
    prev_level: &types::World,
    direction: types::Direction,
) -> Option<types::World> {
    let mut player_move = None;
    let mut level = prev_level.clone();

    // First we find the player and figure out what its new coords will be.
    // if the player is trying to move into a wall we'll do nothing otherwise we'll
    // set the move
    for (index, entity) in level.entities.iter().enumerate() {
        if let types::Entity::Player(player) = entity {
            let new_position = get_new_position(player.position.clone(), &direction);

            match level.board[[new_position.y, new_position.x]] {
                types::Tile::Wall => player_move = None,
                _ => player_move = Some((index, new_position)),
            }
            break;
        }
    }

    let mut soko_box_move = None;
    if let Some((_, player_position)) = player_move.clone() {
        for (index, entity) in level.entities.iter().enumerate() {
            if let types::Entity::SokoBox(soko_box) = entity {
                // if there is a soko_box where the player wants to move see if we can
                // push it.
                if soko_box.position == player_position.clone() {
                    let new_position =
                        get_new_position(soko_box.position.clone(), &direction);

                    if level.is_tile_occupied(&new_position) {
                        // if the tile we are trying to move too is occupied both moves are
                        // invalid.
                        soko_box_move = None;
                        player_move = None;
                    } else {
                        // otherwise move the soko_box
                        soko_box_move = Some((index, new_position.clone()));
                    }
                }
            }
        }
    }

    // resolve the movement
    if let Some((index, new_position)) = player_move {
        if let types::Entity::Player(ref mut player) = &mut level.entities[index] {
            player.position = new_position.clone();
        }
    } else {
        return None;
    }
    if let Some((index, new_position)) = soko_box_move {
        if let types::Entity::SokoBox(ref mut soko_box) = &mut level.entities[index] {
            soko_box.position = new_position.clone();
        }
    }

    Some(level)
}

fn get_new_position(
    position: types::Coordinate,
    direction: &types::Direction,
) -> types::Coordinate {
    match direction {
        types::Direction::Up => types::Coordinate {
            x: position.x,
            y: if position.y > 0 { position.y - 1 } else { 0 },
        },
        types::Direction::Down => types::Coordinate {
            x: position.x,
            y: position.y + 1,
        },
        types::Direction::Left => types::Coordinate {
            x: if position.x > 0 { position.x - 1 } else { 0 },
            y: position.y,
        },
        types::Direction::Right => types::Coordinate {
            x: position.x + 1,
            y: position.y,
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
        // Game state
        KeyCode::Esc => Ok(types::Action::Quit),
        KeyCode::Char('z') | KeyCode::Char('u') => Ok(types::Action::Undo),
        KeyCode::Char('r') => Ok(types::Action::Reset),

        // Movement
        KeyCode::Up | KeyCode::Char('w') => {
            Ok(types::Action::Move(types::Direction::Up))
        }
        KeyCode::Left | KeyCode::Char('a') => {
            Ok(types::Action::Move(types::Direction::Left))
        }
        KeyCode::Down | KeyCode::Char('s') => {
            Ok(types::Action::Move(types::Direction::Down))
        }
        KeyCode::Right | KeyCode::Char('d') => {
            Ok(types::Action::Move(types::Direction::Right))
        }

        // View
        KeyCode::Char('1') => Ok(types::Action::ZoomClose),
        KeyCode::Char('2') => Ok(types::Action::ZoomMiddle),
        KeyCode::Char('3') => Ok(types::Action::ZoomFar),

        _ => Ok(types::Action::None),
    }
}
