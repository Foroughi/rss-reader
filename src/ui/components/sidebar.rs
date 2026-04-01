use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, area: Rect) {
    let items = vec![
        ListItem::new("All"),
        ListItem::new("Rust"),
        ListItem::new("HN"),
        ListItem::new("YouTube"),
    ];

    let list = List::new(items).block(Block::default().title("Filters").borders(Borders::ALL));

    frame.render_widget(list, area);
}
