mod scrambler;
mod timer;
mod ui;

use chrono::Duration;
use crossterm::{
    event::{self, Event, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use scrambler::get_scramble;
use std::io::{self, Stdout};
use timer::Timer;
use tui::{backend::CrosstermBackend, Terminal};
use ui::render_ui;

#[derive(Debug)]
pub struct App {
    timing: bool,
    scramble: String,
    scramble_history: Vec<String>,
    time_history: Vec<Duration>,
    timer: Timer,
    help_shown: bool,
}

impl App {
    fn new() -> App {
        App {
            timing: false,
            scramble: get_scramble(),
            scramble_history: Vec::new(),
            time_history: Vec::new(),
            timer: Timer::new(),
            help_shown: false,
        }
    }

    fn new_scramble(&mut self) {
        self.scramble_history.push(self.scramble.clone());
        self.scramble = get_scramble();
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen,)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;

    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| render_ui(f, &app))?;

        if let Event::Key(e) = event::read()? {
            match e.code {
                event::KeyCode::Char('q') => {
                    return Ok(());
                }
                event::KeyCode::Char('?') => {
                    app.help_shown = !app.help_shown;
                }
                event::KeyCode::Esc => {
                    if app.help_shown == true {
                        app.help_shown = false;
                    } else {
                        app.timer.reset();
                    }
                }
                event::KeyCode::Char(' ') => {
                    app.timing = !app.timing;
                    if app.timing == true {
                        app.timer.start();
                    } else {
                        app.timer.stop();
                        app.time_history.push(app.timer.get_elapsed());
                        app.new_scramble();
                    }
                }
                event::KeyCode::Right => {
                    if e.modifiers == KeyModifiers::SHIFT {
                        app.new_scramble();
                    }
                }
                event::KeyCode::Left => {
                    if e.modifiers == KeyModifiers::SHIFT {
                        if let Some(prev) = app.scramble_history.pop() {
                            app.scramble = prev.clone();
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
