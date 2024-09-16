use std::io;
use std::time::Duration;

use crate::{
    copy_text,
    types::{LevelSelectAction, Model, RunningState},
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
    Frame,
};

pub fn view(model: &mut Model, frame: &mut Frame) {
    let mut view_text = copy_text::LEVEL_SELECT.to_string();

    let _world_index = model.game.world_index;
    // Get all of the names as a vector of strings
    let world_names: Vec<String> = model
        .game
        .worlds
        .iter()
        .enumerate() // Get the index of each world
        .map(|(index, world)| {
            if index == _world_index {
                format!("** {} **\n", world.name) // Highlight the selected world
            } else {
                format!("   {}\n", world.name) // Normal formatting for other worlds
            }
        })
        .collect();

    // TODO: @Lee I tried to do some pretty simple logic but i dont think its in a rusty way. im going to ignore this problem
    // for now lol
    // Print 11 rows, with selected in the middle, or otherwise if needed for shorter lists
    let num_of_rows = std::cmp::min(11, world_names.len());

    let end_index = (_world_index + num_of_rows).min(world_names.len());

    // Get the slice of names from the vector
    let world_names_slice = &world_names[_world_index..end_index];
    // Join the slice into a single string
    let world_names_joined = world_names_slice.join("");

    // Push the concatenated string to view_text
    view_text.push_str(&world_names_joined);

    frame.render_widget(Paragraph::new(view_text), frame.area());
}

pub fn update(model: &mut Model, msg: LevelSelectAction) -> Option<LevelSelectAction> {
    match msg {
        LevelSelectAction::Quit => {
            model.running_state = RunningState::Menu;
        }
        LevelSelectAction::Select => {
            model.running_state = RunningState::Game;
        }
        LevelSelectAction::Up => {
            model.game.decrement_level();
        }
        LevelSelectAction::Down => {
            model.game.increment_level();
        }
    };
    None
}

pub fn handle_event(_: &Model) -> io::Result<Option<LevelSelectAction>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

// Handle todo: repeating same key logic is kinda clunky. the idea of "up" is the same across the board bro
pub fn handle_key(key: event::KeyEvent) -> Option<LevelSelectAction> {
    match key.code {
        KeyCode::Esc => Some(LevelSelectAction::Quit),
        KeyCode::Enter | KeyCode::Char(' ') => Some(LevelSelectAction::Select),
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            Some(LevelSelectAction::Up)
        }
        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
            Some(LevelSelectAction::Down)
        }
        _ => None,
    }
}
