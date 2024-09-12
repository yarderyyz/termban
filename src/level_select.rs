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

pub fn view(_model: &mut Model, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(copy_text::LEVEL_SELECT.to_string()),
        frame.area(),
    );
}

pub fn update(model: &mut Model, msg: LevelSelectAction) -> Option<LevelSelectAction> {
    match msg {
        LevelSelectAction::Quit => {
            model.running_state = RunningState::Menu;
        }
        LevelSelectAction::Select => {
            model.running_state = RunningState::Game;
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

pub fn handle_key(key: event::KeyEvent) -> Option<LevelSelectAction> {
    match key.code {
        KeyCode::Esc => Some(LevelSelectAction::Quit),
        KeyCode::Enter | KeyCode::Char(' ') => Some(LevelSelectAction::Select),
        _ => None,
    }
}
