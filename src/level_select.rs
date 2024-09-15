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
    // Get list of levels by name
    // let names = model.game.worlds.len();
    // let worlds_len: String = model.game.worlds.clone().len().to_string();
    // let current_level = model.game.world_index.to_string();
    // get range of the levels to display (lets say 8) // might have to be all cool n deldy
    // append only those hot hot babes
    // view_text.push_str(&worlds_len);
    // view_text.push_str(&current_level);
    view_text.push('\n');

    let selected_index = model.game.world_index;
    // Get all of the names as a vector of strings
    let world_names: Vec<String> = model
        .game
        .worlds
        .iter()
        .enumerate() // Get the index of each world
        .map(|(index, world)| {
            if index == selected_index {
                format!("** {} **\n", world.name) // Highlight the selected world
            } else {
                format!("   {}\n", world.name) // Normal formatting for other worlds
            }
        })
        .collect();

    // Define the starting index and maximum number of names to show
    let max_names = 10;
    let end_index = (selected_index + max_names).min(world_names.len());

    // Get the slice of names from the vector
    let world_names_slice = &world_names[selected_index..end_index];
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
