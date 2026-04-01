use ratatui::prelude::*;

pub struct LayoutChunks {
    pub header: Rect,
    pub sidebar: Rect,
    pub main: Rect,
    pub status: Rect,
}

pub fn create_layout(frame: &Frame) -> LayoutChunks {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // header
            Constraint::Min(1),    // body
            Constraint::Length(1), // status
        ])
        .split(frame.size());

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30), // sidebar
            Constraint::Min(1),     // main
        ])
        .split(vertical[1]);

    LayoutChunks {
        header: vertical[0],
        sidebar: body[0],
        main: body[1],
        status: vertical[2],
    }
}
