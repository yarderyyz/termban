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
    let worlds_len: String = model.game.worlds.clone().len().to_string();
    let current_level = model.game.world_index.to_string();
    // get range of the levels to display (lets say 8) // might have to be all cool n deldy
    // append only those hot hot babes
    view_text.push_str(&worlds_len);
    view_text.push_str(&current_level);
    view_text.push('\n');

    // get all of the names as a string
    let world_names: String = model
        .game
        .worlds
        .iter() // Create an iterator over the Vec
        .map(|world| format!("{}\n", world.name)) // Append a newline to each name
        .collect::<Vec<_>>() // Collect into a Vec<String>
        .join("");
    view_text.push_str(&world_names);

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
        LevelSelectAction::Up => {}
        LevelSelectAction::Down => {}
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
