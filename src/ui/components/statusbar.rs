use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, area: Rect) {
    let text = "q: quit | ↑↓: navigate | o: open";

    let bar = Paragraph::new(text).style(Style::default().bg(Color::DarkGray));

    frame.render_widget(bar, area);
}
