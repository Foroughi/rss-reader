use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

use crate::domain::item::Item;

pub struct Ui {
    items: Vec<Item>,
    selected: usize,
}

impl Ui {
    pub fn new(items: Vec<Item>) -> Self {
        Self {
            items,
            selected: 0,
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
        let items: Vec<ListItem> = self.items.iter().map(|i| {
            let line = format!(
                "[{:?}] ({}) {}",
                i.source,
                i.score.unwrap_or(0),
                i.title
            );
            ListItem::new(line)
        }).collect();

        let list = List::new(items)
            .block(Block::default().title("RSS Hub").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        let mut state = ListState::default();
        state.select(Some(self.selected));

        f.render_stateful_widget(list, f.size(), &mut state.clone());
    }
}
