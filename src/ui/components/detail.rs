use ratatui::{prelude::*, widgets::*};

use crate::domain::item::Item;

use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, area: Rect, item: &Item, scroll: u16) {
    let raw = item.content.as_deref().unwrap_or("No description");

    let cleaned = html2text::from_read(raw.as_bytes(), area.width as usize);

    let paragraph = Paragraph::new(format!("{}\n\n{}", item.title, cleaned))
        .block(Block::default().title("Article").borders(Borders::ALL))
        .wrap(Wrap { trim: true })
        .scroll((scroll, 0)); // 👈 THIS is the magic

    frame.render_widget(paragraph, area);
}
