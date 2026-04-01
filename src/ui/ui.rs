use std::io;

use crate::services::extract;
use crate::services::fetch;
use crate::ui::{
    components::{detail, header, main, sidebar, statusbar},
    layout::create_layout,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::*, widgets::*};

use crate::domain::item::Item;
enum Mode {
    List,
    Detail,
    Filters,
}

pub struct Ui {
    items: Vec<Item>,
    selected: usize,
    scroll: u16,
    filter: usize,
    mode: Mode,
    current_item: Option<Item>,
}

impl Ui {
    pub fn new(items: Vec<Item>) -> Self {
        Self {
            items,
            selected: 0,
            mode: Mode::List,
            current_item: None,
            scroll: 0,
            filter: 0,
        }
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
                match self.mode {
                    Mode::Detail => match key.code {
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.scroll = self.scroll.saturating_add(1);
                        }

                        KeyCode::Up | KeyCode::Char('k') => {
                            self.scroll = self.scroll.saturating_sub(1);
                        }
                        KeyCode::Char('o') => {
                            if let Some(item) = self.items.get(self.selected) {
                                let _ = open::that(&item.url);
                            }
                        }
                        KeyCode::Esc => {
                            self.mode = Mode::List;
                            self.scroll = 0;
                        }

                        _ => {}
                    },

                    Mode::List => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.selected = self.selected.saturating_add(1);
                        }
                        KeyCode::Char('o') => {
                            if let Some(item) = self.items.get(self.selected) {
                                let _ = open::that(&item.url);
                            }
                        }

                        KeyCode::Esc => {
                            self.mode = Mode::Filters;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.selected = self.selected.saturating_sub(1);
                        }

                        KeyCode::Enter => {
                            if let Some(item) = self.items.get(self.selected) {
                                self.mode = Mode::Detail;
                                self.scroll = 0;
                                self.current_item = Some(item.clone());
                            }
                        }

                        _ => {}
                    },

                    Mode::Filters => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.filter = self.filter.saturating_add(1);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.filter = self.filter.saturating_sub(1);
                        }

                        KeyCode::Enter => {
                            self.mode = Mode::List;
                        }

                        _ => {}
                    },
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
        sidebar::render(f, chunks.sidebar, &self.items, self.filter);
        match self.mode {
            Mode::List => {
                main::render(f, chunks.main, &self.items, self.selected);
            }
            Mode::Detail => {
                if let Some(item) = &self.current_item {
                    detail::render(f, chunks.main, &item, self.scroll);
                }
            }

            Mode::Filters => {}
        }

        statusbar::render(f, chunks.status);
    }
}
