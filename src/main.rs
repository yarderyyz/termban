use palette::{IntoColor, Okhsv, Srgb};
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
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

use std::fs::File;
use std::io::{self, stdout, Read};

mod colors;
mod soko_loader;
mod types;

impl Widget for types::Level {
    #[allow(clippy::cast_precision_loss, clippy::similar_names)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut row_pixels;
        let mut row_iter = self.map.outer_iter();
        let mut yi: usize = 0;
        while let Some(row_top) = row_iter.next() {
            let maybe_row_bottom = row_iter.next();

            let top_colors;
            let bottom_colors;
            match maybe_row_bottom {
                Some(row_bottom) => {
                    top_colors = row_top.map(|ent| ent.color());
                    bottom_colors = row_bottom.map(|ent| ent.color());
                    row_pixels = top_colors.iter().zip(bottom_colors.iter());
                }
                None => {
                    top_colors = row_top.map(|ent| ent.color());
                    bottom_colors = row_top.map(|_| None);
                    row_pixels = top_colors.iter().zip(bottom_colors.iter());
                }
            }

            for (xi, (fg, bg)) in row_pixels.enumerate() {
                let curs = &mut buf[(xi as u16 + area.x, yi as u16 + area.y)];
                curs.set_char('▀');
                if let Some(fg) = fg {
                    curs.set_fg(fg.clone());
                }
                if let Some(bg) = bg {
                    curs.set_bg(bg.clone());
                }
            }

            yi += 1;
        }
        for entity in self.entities {
            match entity {
                // TODO: Do this with traits or something, These render the exact same
                types::Entity::Player(player) => {
                    let px_x = player.coords.x as u16 + area.x;
                    let px_y = (player.coords.y / 2) as u16 + area.y;
                    if area.contains(Position {
                        x: px_x as u16,
                        y: px_y as u16,
                    }) {
                        let curs = &mut buf[(px_x, px_y)];
                        if player.coords.y % 2 == 0 {
                            curs.set_fg(player.color);
                        } else {
                            curs.set_bg(player.color);
                        }
                    }
                }
                types::Entity::Chest(chest) => {
                    let px_x = chest.coords.x;
                    let px_y = chest.coords.y / 2;
                    let curs = &mut buf[(px_x as u16 + area.x, px_y as u16 + area.y)];
                    if chest.coords.y % 2 == 0 {
                        curs.set_fg(chest.color);
                    } else {
                        curs.set_bg(chest.color);
                    }
                }
            }
        }
    }
}

pub fn color_from_oklab(hue: f32, saturation: f32, value: f32) -> Color {
    let color: Srgb = Okhsv::new(hue, saturation, value).into_color();
    let color = color.into_format();
    Color::Rgb(color.red, color.green, color.blue)
}

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
                // Iterate over mutable references to entities
                for entity in level.entities.iter_mut() {
                    if let types::Entity::Player(player) = entity {
                        let new_coords = match direction {
                            types::Direction::Up => types::Coordinates {
                                x: player.coords.x,
                                y: if player.coords.y > 0 {
                                    player.coords.y - 1
                                } else {
                                    0
                                },
                            },
                            types::Direction::Down => types::Coordinates {
                                x: player.coords.x,
                                y: player.coords.y + 1,
                            },
                            types::Direction::Left => types::Coordinates {
                                x: if player.coords.x > 0 {
                                    player.coords.x - 1
                                } else {
                                    0
                                },
                                y: player.coords.y,
                            },
                            types::Direction::Right => types::Coordinates {
                                x: player.coords.x + 1,
                                y: player.coords.y,
                            },
                        };
                        match level.map[[new_coords.y, new_coords.x]] {
                            types::Tile::Wall => {} // Don't move its into a wall
                            _ => {
                                player.coords.x = new_coords.x;
                                player.coords.y = new_coords.y;
                            }
                        }
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

fn handle_events() -> io::Result<types::Action> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q')
            {
                return Ok(types::Action::Quit);
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('w')
            {
                return Ok(types::Action::Move(types::Direction::Up));
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('s')
            {
                return Ok(types::Action::Move(types::Direction::Down));
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('a')
            {
                return Ok(types::Action::Move(types::Direction::Left));
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('d')
            {
                return Ok(types::Action::Move(types::Direction::Right));
            }
        }
    }
    Ok(types::Action::None)
}
