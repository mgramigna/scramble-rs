use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::App;

pub fn render_ui(f: &mut Frame<CrosstermBackend<Stdout>>, app: &App) {
    let size = f.size();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
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
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(main_chunks[1]);

    let scramble_box = Block::default().borders(Borders::ALL).title("Scramble");

    let scramble_text = Paragraph::new(app.scramble.clone())
        .block(scramble_box)
        .alignment(Alignment::Center);

    f.render_widget(scramble_text, main_chunks[0]);

    let timer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(45),
                Constraint::Percentage(10),
                Constraint::Percentage(45),
            ]
            .as_ref(),
        )
        .split(content_chunks[1]);

    let timer_box = Block::default().borders(Borders::ALL);
    f.render_widget(timer_box, content_chunks[1]);

    if app.help_shown {
        let help_spans: Vec<ListItem> = vec![
            ListItem::new(Span::raw("Help:")),
            ListItem::new(Span::raw("")),
            ListItem::new(Span::raw("SPACE - start timer")),
            ListItem::new(Span::raw("SHIFT+LEFT ARROW - previous scramble")),
            ListItem::new(Span::raw("SHIFT+RIGHT ARROW - previous scramble")),
            ListItem::new(Span::raw("? - toggle help")),
            ListItem::new(Span::raw("q - quit")),
        ];

        let help_paragraph = List::new(help_spans).block(Block::default().borders(Borders::ALL));
        f.render_widget(help_paragraph, content_chunks[1]);
    } else {
        let timer_text = if app.timer.is_stopped() {
            app.timer.to_string()
        } else {
            String::from("SOLVE")
        };
        let timer_paragraph = Paragraph::new(timer_text)
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(timer_paragraph, timer_chunks[1]);
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
