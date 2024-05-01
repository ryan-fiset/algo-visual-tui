use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(Constraint::from_percentages([8, 88, 4]))
        .split(frame.size());

    let main_info = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_percentages([33, 33, 33]))
        .split(chunks[0]);

    // Algorithm
    frame.render_widget(
        Paragraph::new("Bubble Sort")
            .block(
                Block::default()
                    .title("Algorithm [C]")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center),
        main_info[0],
    );

    // Data Length
    frame.render_widget(
        Paragraph::new("50")
            .block(
                Block::default()
                    .title("Data Length [J/K]")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center),
        main_info[1],
    );

    // Tick Rate
    frame.render_widget(
        Paragraph::new("10")
            .block(
                Block::default()
                    .title("Tick Rate [H/L]")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center),
        main_info[2],
    );

    // Graph
    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::default()
                    .title("Graph")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center),
        chunks[1],
    );

    // Key hints
    frame.render_widget(
        Paragraph::new("[Enter]: Start/Stop Sorting, [R]: Shuffle Data"),
        chunks[2],
    );
}
