use std::fs::File;
use std::io::{self, Read};

mod colors;
mod copy_text;
mod menu;
mod render;
mod soko_game;
mod soko_loader;
mod sprites;
mod types;

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    let ban_filename = "./resources/levels/micro2.ban";
    // TODO: actually handle errors here
    //
    let worlds = read_file(ban_filename)
        .map(|contents| soko_loader::parse_sokoban_worlds(&contents).unwrap())
        .unwrap();
    let starting_world_i = 0;
    let mut current_world_i = starting_world_i;
    let game_window = types::GameWindow {
        world: worlds[starting_world_i].clone(),
        zoom: types::Zoom::Far,
        debug: Vec::new(),
    };
    let mut model = types::Model {
        running_state: types::RunningState::Menu,
        game: types::Game {
            history: Vec::new(),
            window: game_window,
        },
    };

    loop {
        match model.running_state {
            types::RunningState::Done => {
                break;
            }
            types::RunningState::Menu => {
                terminal.draw(|f| menu::view(&mut model, f))?;
                // Handle events and map to a Message
                let mut current_msg = menu::handle_event(&model)?;

                // Process updates as long as they return a non-None message
                while current_msg.is_some() {
                    current_msg = menu::update(&mut model, current_msg.unwrap());
                }
            }
            types::RunningState::Game => {
                terminal.draw(|f| soko_game::view(&mut model, f))?;

                // Handle events and map to a Message
                let mut current_msg = soko_game::handle_event(&mut model)?;

                // This has to happen before the While loop below, for some reason.
                // @lee-gauthier? Maybe this needs to be refactored into update or something --
                // but the model, worlds, and current world lives in main eh?
                // We should do some light refactoring?
                if let Some(types::GameAction::Win) = current_msg {
                    current_world_i += 1;
                    model.game.window.world = worlds[current_world_i].clone();

                    // Erase your history before resetting, lest you restart your completed level
                    soko_game::reset_history(&mut model.game);
                    soko_game::reset_world(&mut model.game);
                    continue;
                }

                // Process updates as long as they return a non-None message
                while current_msg.is_some() {
                    current_msg = soko_game::update(&mut model, current_msg.unwrap());
                }
            }
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

mod tui {
    use ratatui::{
        backend::{Backend, CrosstermBackend},
        crossterm::{
            terminal::{
                disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
                LeaveAlternateScreen,
            },
            ExecutableCommand,
        },
        Terminal,
    };
    use std::io;
    use std::{io::stdout, panic};

    pub fn init_terminal() -> io::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> io::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}
