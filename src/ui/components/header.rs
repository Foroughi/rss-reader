use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new("RSS Hub").style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(header, area);
}
