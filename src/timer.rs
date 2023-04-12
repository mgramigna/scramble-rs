use chrono::{DateTime, Duration, Local};
use std::fmt::Display;

#[derive(Debug)]
pub struct Timer {
    duration: Duration,
    started_at: Option<DateTime<Local>>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            duration: Duration::zero(),
            started_at: None,
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.started_at.is_none()
    }

    pub fn start(&mut self) {
        self.duration = Duration::zero();
        self.started_at = Some(Local::now());
    }

    pub fn stop(&mut self) {
        if let Some(started_at) = self.started_at {
            self.duration = self.duration + Local::now().signed_duration_since(started_at);
            self.started_at = None
        }
    }

    pub fn reset(&mut self) {
        if self.is_stopped() {
            self.started_at = None;
            self.duration = Duration::zero();
        }
    }

    pub fn get_elapsed(&self) -> Duration {
        if let Some(started_at) = self.started_at {
            Local::now().signed_duration_since(started_at) + self.duration
        } else {
            self.duration
        }
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elapsed = self.get_elapsed();
        write!(
            f,
            "{}.{}",
            elapsed.num_seconds(),
            elapsed.num_milliseconds()
        )
    }
}
