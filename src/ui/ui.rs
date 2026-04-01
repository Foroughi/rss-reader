use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::*, widgets::*};

use crate::ui::{
    components::{header, main, sidebar, statusbar},
    layout::create_layout,
};

use crate::domain::item::Item;

pub struct Ui {
    items: Vec<Item>,
    selected: usize,
}

impl Ui {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items, selected: 0 }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| self.draw(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('o') => {
                        if let Some(item) = self.items.get(self.selected) {
                            let _ = open::that(&item.url);
                        }
                    }

                    KeyCode::Down => {
                        if self.selected + 1 < self.items.len() {
                            self.selected += 1;
                        }
                    }

                    KeyCode::Up => {
                        if self.selected > 0 {
                            self.selected -= 1;
                        }
                    }

                    _ => {}
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn draw(&self, f: &mut Frame) {
        let chunks = create_layout(f);

        header::render(f, chunks.header);
        sidebar::render(f, chunks.sidebar);
        main::render(f, chunks.main, &self.items, self.selected);
        statusbar::render(f, chunks.status);
    }
}
