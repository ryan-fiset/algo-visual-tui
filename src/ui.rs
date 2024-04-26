use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(frame.size());
    frame.render_widget(
        Tabs::new(app.tab_list.clone())
            .block(
                Block::default()
                    .title("Algorithm")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .select(app.current_tab),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::default()
                    .title("Sort")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center),
        layout[1],
    )
}
