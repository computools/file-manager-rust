use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListDirection, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Wrap,
    },
    Frame,
};

use crate::app::App;

pub static HELP: &str = include_str!("../resources/help");

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let mut main_area = frame.area();
    let mut help_area = frame.area();

    if app.show_help {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(frame.area());

        main_area = layout[0];
        help_area = layout[1];
    }

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let file_names = app.get_file_names();
    let mut files_scrollbar_state =
        ScrollbarState::new(file_names.len()).position(app.get_files_selected());

    let [main_area, preview_area] =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
            .areas(main_area);

    let list = List::new(file_names)
        .block(Block::bordered().title("List"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">> ")
        .highlight_spacing(HighlightSpacing::Always)
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(
        list.block(
            Block::bordered()
                .title(Line::from(vec![
                    Span::styled("F", Style::new().underlined()),
                    "iles".into(),
                ]))
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        main_area,
        &mut app.files_state,
    );

    frame.render_stateful_widget(
        scrollbar.clone(),
        main_area.inner(Margin {
            // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            vertical: 1,
            horizontal: 0,
        }),
        &mut files_scrollbar_state,
    );

    frame.render_widget(
        Paragraph::new(app.get_preview())
            .block(
                Block::bordered()
                    .title("Preview")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .wrap(Wrap { trim: true }),
        preview_area,
    );

    if app.show_help {
        let mut help_scrollbar_state =
            ScrollbarState::new(HELP.lines().count()).position(app.help_offset);

        frame.render_widget(
            Paragraph::new(HELP)
                .block(
                    Block::bordered()
                        .title(Line::from(vec![
                            "Help (".into(),
                            Span::styled("?", Style::new().underlined()),
                            ")".into(),
                        ]))
                        .title_alignment(Alignment::Center)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .wrap(Wrap { trim: true })
                .scroll((app.help_offset as u16, 0)),
            help_area,
        );

        frame.render_stateful_widget(
            scrollbar,
            help_area.inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            &mut help_scrollbar_state,
        );
    }
}
