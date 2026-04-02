use std::collections::HashSet;
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
pub enum Mode {
    List,
    Detail,
    Filters,
}

pub struct Ui {
    items: Vec<Item>,
    sources: Vec<String>,
    selected: usize,
    scroll: u16,
    filter: usize,
    mode: Mode,
    current_item: Option<Item>,
}

impl Ui {
    pub fn new(items: Vec<Item>) -> Self {
        let mut set = std::collections::HashSet::new();

        for item in items.clone() {
            for tag in &item.tags {
                set.insert(tag.clone());
            }
        }

        let mut tags: Vec<String> = set.into_iter().collect();
        tags.sort();
        tags.insert(0, "All".to_string());

        Self {
            items,
            selected: 0,
            mode: Mode::Filters,
            current_item: None,
            scroll: 0,
            sources: tags,
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
                        KeyCode::Esc | KeyCode::Char('h') => {
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

                        KeyCode::Char('h') | KeyCode::Left => {
                            self.mode = Mode::Filters;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.selected = self.selected.saturating_sub(1);
                        }

                        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
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

                            self.selected = 0;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.filter = self.filter.saturating_sub(1);

                            self.selected = 0;
                        }

                        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                            self.mode = Mode::List;
                            self.selected = 0;
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
        let filter = self.sources[self.filter].clone();
        let items: Vec<Item> = self
            .items
            .iter()
            .filter(|item| filter == "All" || item.tags.contains(&filter))
            .cloned()
            .collect();

        header::render(f, chunks.header);
        sidebar::render(f, chunks.sidebar, &self.sources, self.filter);
        match self.mode {
            Mode::List => {
                main::render(f, chunks.main, &items, self.selected, filter);
            }
            Mode::Detail => {
                if let Some(item) = &self.current_item {
                    detail::render(f, chunks.main, &item, self.scroll);
                }
            }

            Mode::Filters => {
                main::render(f, chunks.main, &items, self.selected, filter);
            }
        }

        statusbar::render(f, chunks.status);
    }
}
