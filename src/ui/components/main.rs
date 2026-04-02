use crate::{domain::item::Item, ui::ui::Mode};
use crossterm::style::style;
use ratatui::{prelude::*, widgets::*};
pub fn render(frame: &mut Frame, area: Rect, items: &[Item], selected: usize, filter: String) {
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|i| {
            let mut line = format!("{}", i.title);

            if filter == "All" {
                line = format!("[{}] {}", i.tags[0], i.title);
            }
            ListItem::new(line)
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().title("Feed").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::Black))
        .highlight_symbol("");

    let mut state = ListState::default();
    state.select(Some(selected));

    frame.render_stateful_widget(list, area, &mut state);
}
