use std::io;
use std::time::Duration;

use crate::types::{
    Coordinate, Direction, Entity, GameAction, Model, RunningState, Tile, World, Zoom,
};
use ratatui::prelude::*;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn view(model: &mut Model, frame: &mut Frame) {
    let game_window = &mut model.game.window;

    let main_area = frame.area();

    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(main_area);

    let title = &game_window.world.name;
    let outer_left_block = Block::bordered().title(title.clone());
    let inner_left = outer_left_block.inner(left_area);

    frame.render_widget(outer_left_block, left_area);
    frame.render_widget(game_window.clone(), inner_left);

    let text = game_window.debug.join("\n");
    frame.render_widget(
        Paragraph::new(text).block(Block::bordered().title("debug")),
        right_area,
    );
}

pub fn handle_key(key: event::KeyEvent) -> Option<GameAction> {
    match key.code {
        // Game state
        KeyCode::Esc => Some(GameAction::Quit),
        KeyCode::Char('z') | KeyCode::Char('u') => Some(GameAction::Undo),
        KeyCode::Char('r') => Some(GameAction::Reset),

        // Movement
        KeyCode::Up | KeyCode::Char('w') => Some(GameAction::Move(Direction::Up)),
        KeyCode::Left | KeyCode::Char('a') => Some(GameAction::Move(Direction::Left)),
        KeyCode::Down | KeyCode::Char('s') => Some(GameAction::Move(Direction::Down)),
        KeyCode::Right | KeyCode::Char('d') => Some(GameAction::Move(Direction::Right)),

        // View
        KeyCode::Char('1') => Some(GameAction::ZoomClose),
        KeyCode::Char('2') => Some(GameAction::ZoomMiddle),
        KeyCode::Char('3') => Some(GameAction::ZoomFar),

        _ => Some(GameAction::None),
    }
}

pub fn update(model: &mut Model, msg: GameAction) -> Option<GameAction> {
    let game = &mut model.game;
    match msg {
        GameAction::Quit => model.running_state = RunningState::Menu,
        GameAction::Move(direction) => {
            if let Some(new_level) = handle_move(&game.window.world, direction) {
                game.history.push(game.window.world.clone());
                game.window.world = new_level;
            }
        }
        GameAction::Undo => {
            if let Some(prev_level) = game.history.pop() {
                game.window.world = prev_level;
            }
        }
        GameAction::Reset => {
            game.history.push(game.window.world.clone());
            if let Some(prev_level) = game.history.first() {
                game.window.world = prev_level.clone();
            }
        }
        GameAction::ZoomClose => game.window.zoom = Zoom::Close,
        GameAction::ZoomMiddle => game.window.zoom = Zoom::Middle,
        GameAction::ZoomFar => game.window.zoom = Zoom::Far,
        GameAction::None => {}
    };
    None
}

pub fn handle_event(model: &mut Model) -> io::Result<Option<GameAction>> {
    let window = &mut model.game.window;
    window.debug = Vec::new();

    for entity in window.world.entities.iter() {
        if let Entity::Player(player) = entity {
            window.debug.push(format!("{:?}", player.position.clone()));
        }
    }
    window
        .debug
        .push(format!("{:?}", &window.world.board.dim()));

    if window.world.is_sokoban_solved() {
        window.debug.push("You win!".to_string());
    }

    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_move(prev_level: &World, direction: Direction) -> Option<World> {
    let mut player_move = None;
    let mut level = prev_level.clone();

    // First we find the player and figure out what its new coords will be.
    // if the player is trying to move into a wall we'll do nothing otherwise we'll
    // set the move
    for (index, entity) in level.entities.iter().enumerate() {
        if let Entity::Player(player) = entity {
            let new_position = get_new_position(player.position.clone(), &direction);

            match level.board[[new_position.y, new_position.x]] {
                Tile::Wall => player_move = None,
                _ => player_move = Some((index, new_position)),
            }
            break;
        }
    }

    let mut soko_box_move = None;
    if let Some((_, player_position)) = player_move.clone() {
        for (index, entity) in level.entities.iter().enumerate() {
            if let Entity::SokoBox(soko_box) = entity {
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
        if let Entity::Player(ref mut player) = &mut level.entities[index] {
            player.position = new_position.clone();
        }
    } else {
        return None;
    }
    if let Some((index, new_position)) = soko_box_move {
        if let Entity::SokoBox(ref mut soko_box) = &mut level.entities[index] {
            soko_box.position = new_position.clone();
        }
    }

    Some(level)
}

fn get_new_position(position: Coordinate, direction: &Direction) -> Coordinate {
    match direction {
        Direction::Up => Coordinate {
            x: position.x,
            y: if position.y > 0 { position.y - 1 } else { 0 },
        },
        Direction::Down => Coordinate {
            x: position.x,
            y: position.y + 1,
        },
        Direction::Left => Coordinate {
            x: if position.x > 0 { position.x - 1 } else { 0 },
            y: position.y,
        },
        Direction::Right => Coordinate {
            x: position.x + 1,
            y: position.y,
        },
    }
}
