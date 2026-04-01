use crate::domain::item::Item;
use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, area: Rect, items: &[Item], selected: usize) {
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|i| {
            let line = format!("[{:?}] ({}) {}", i.source, i.score.unwrap_or(0), i.title);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().title("Feed").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(Some(selected));

    frame.render_stateful_widget(list, area, &mut state);
}
