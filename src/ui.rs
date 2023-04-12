use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::App;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(layout[1])[1]
}

pub fn render_ui(f: &mut Frame<CrosstermBackend<Stdout>>, app: &App) {
    let size = f.size();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(5),  // Scramble
                Constraint::Percentage(90), // Past times and timer
                Constraint::Percentage(5),  // Footer
            ]
            .as_ref(),
        )
        .split(size);

    let footer_box = Block::default().borders(Borders::ALL);
    let footer_paragraph = Paragraph::new("Press ? for help")
        .block(footer_box)
        .alignment(Alignment::Center);

    f.render_widget(footer_paragraph, main_chunks[2]);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25), // Past times
                Constraint::Percentage(75), // Timer
            ]
            .as_ref(),
        )
        .split(main_chunks[1]);

    let scramble_box = Block::default().borders(Borders::ALL).title("Scramble");

    let scramble_text = Paragraph::new(app.scramble.clone())
        .block(scramble_box)
        .alignment(Alignment::Center);

    f.render_widget(scramble_text, main_chunks[0]);

    let timer_box = Block::default().borders(Borders::ALL);
    f.render_widget(timer_box, content_chunks[1]);

    if app.help_shown {
        let area = centered_rect(50, 40, content_chunks[1]);
        f.render_widget(Clear, area); //this clears out the background

        let help_spans: Vec<ListItem> = vec![
            ListItem::new(Span::raw("")),
            ListItem::new(Span::raw("SPACE - start timer")),
            ListItem::new(Span::raw("")),
            ListItem::new(Span::raw("SHIFT+LEFT ARROW - previous scramble")),
            ListItem::new(Span::raw("")),
            ListItem::new(Span::raw("SHIFT+RIGHT ARROW - previous scramble")),
            ListItem::new(Span::raw("")),
            ListItem::new(Span::raw("? - toggle help")),
            ListItem::new(Span::raw("")),
            ListItem::new(Span::raw("q - quit")),
        ];

        let help_paragraph =
            List::new(help_spans).block(Block::default().title("Help").borders(Borders::ALL));

        f.render_widget(help_paragraph, area);
    } else {
        let timer_text = if app.timer.is_stopped() {
            app.timer.to_string()
        } else {
            String::from("SOLVE")
        };

        let timer_paragraph = Paragraph::new(timer_text).alignment(Alignment::Center);

        let avg_five_text: String;

        if let Some(d) = app.get_avg_five() {
            avg_five_text = format!("{}.{}", d.num_seconds(), d.num_milliseconds());
        } else {
            avg_five_text = "DNF".to_string();
        }

        let avg_of_five_paragraph =
            Paragraph::new(format!("AO5: {}", avg_five_text)).alignment(Alignment::Center);

        let area = centered_rect(50, 10, content_chunks[1]);

        let timer_sub_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                vec![
                    Constraint::Percentage(33),
                    Constraint::Percentage(33), // Timer
                    Constraint::Percentage(33), // AO5
                ]
                .as_ref(),
            )
            .split(area);

        f.render_widget(timer_paragraph, timer_sub_chunks[1]);
        f.render_widget(avg_of_five_paragraph, timer_sub_chunks[2]);
    }

    let past_times: Vec<ListItem> = app
        .time_history
        .iter()
        .rev()
        .map(|d| {
            let content = vec![Spans::from(Span::raw(format!(
                "{}.{}",
                d.num_seconds(),
                d.num_milliseconds()
            )))];
            ListItem::new(content)
        })
        .collect();

    let past_times_list =
        List::new(past_times).block(Block::default().borders(Borders::ALL).title("Past Times"));

    f.render_widget(past_times_list, content_chunks[0]);
}
