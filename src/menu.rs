use crate::copy_text;
use crate::types::{MenuAction, Model, RunningState};
use std::time::Duration;
use std::{fs, io};

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
    Frame,
};

pub fn view(_model: &mut Model, frame: &mut Frame) {
    frame.render_widget(Paragraph::new(copy_text::MENU.to_string()), frame.area());
}

pub fn update(model: &mut Model, msg: MenuAction) -> Option<MenuAction> {
    match msg {
        MenuAction::StartGame => {
            model.running_state = RunningState::LevelSelect;
        }
        MenuAction::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::Done;
        }
        MenuAction::EraseSaveData => {
            // When dev/user deletes a save file, put them back on world 1
            model.game.world_index = 0;
            model.game.refresh_window();
            delete_save_file();
        }
    };
    None
}

fn delete_save_file() {
    let save_file = "saves.toml";
    match fs::remove_file(save_file) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

/// Convert Event to Message
///
/// We don't need to pass in a `model` to this function in this example
/// but you might need it as your project evolves
pub fn handle_event(_: &Model) -> io::Result<Option<MenuAction>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

pub fn handle_key(key: event::KeyEvent) -> Option<MenuAction> {
    match key.code {
        KeyCode::Enter | KeyCode::Char(' ') => Some(MenuAction::StartGame),
        KeyCode::Esc => Some(MenuAction::Quit),
        KeyCode::Delete => Some(MenuAction::EraseSaveData),
        _ => None,
    }
}
