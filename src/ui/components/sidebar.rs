use crate::domain::item::Item;
use ratatui::{prelude::*, widgets::*};
use std::collections::HashSet;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub fn render(frame: &mut Frame, area: Rect, items: &[Item], selected: usize) {
    let mut set = std::collections::HashSet::new();

    for item in items {
        for tag in &item.tags {
            set.insert(tag.clone());
        }
    }

    let mut tags: Vec<String> = set.into_iter().collect();
    tags.sort();
    tags.insert(0, "All".to_string());

    let list_items: Vec<ListItem> = tags.iter().map(|t| ListItem::new(t.as_str())).collect();

    let list = List::new(list_items)
        .block(Block::default().title("Feeds").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(Some(selected));

    frame.render_stateful_widget(list, area, &mut state);
}
