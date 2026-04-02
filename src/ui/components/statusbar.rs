use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, area: Rect) {
    let text = " q: quit | ↑↓: navigate | o: open";

    let bar = Paragraph::new(text).style(Style::default().bg(Color::Blue).fg(Color::Black));

    frame.render_widget(bar, area);
}
