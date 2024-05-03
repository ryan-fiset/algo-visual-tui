use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, BorderType, Borders, Paragraph},
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
        Paragraph::new(app.data_length.to_string())
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
        Paragraph::new(app.tick_rate.to_string())
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
        BarChart::default()
            .block(
                Block::default()
                    .title("Graph")
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center),
            )
            .data(&app.bar_data)
            .bar_width(1)
            .bar_gap(1)
            .bar_style(Style::default().fg(Color::White))
            .label_style(
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            )
            .value_style(Style::default().fg(Color::White).bg(Color::White)),
        chunks[1],
    );

    // Key hints
    frame.render_widget(
        Paragraph::new("[Enter]: Start/Stop Sorting, [R]: Shuffle Data"),
        chunks[2],
    );
}
