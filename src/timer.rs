use crate::common::now;
use crate::timer::TimerState::*;

#[derive(Debug, PartialEq)]
enum TimerState {
    NotStarted,
    Running,
    Stopped,
}

#[derive(Debug)]
pub struct Timer {
    start_time: u64,
    elapsed: u64,
    state: TimerState,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: 0,
            elapsed: 0,
            state: NotStarted,
        }
    }

    pub fn new_started() -> Self {
        Timer {
            start_time: now(),
            elapsed: 0,
            state: Running,
        }
    }

    pub fn start(&mut self) {
        if self.state == NotStarted {
            self.start_time = now();
            self.state = Running;
        }
    }

    pub fn stop(&mut self) {
        if self.state == Running {
            self.elapsed = now() - self.start_time;
            self.state = Stopped;
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        match self.state {
            NotStarted => 0,
            Running => now() - self.start_time,
            Stopped => self.elapsed,
        }
    }

    pub fn reset(&mut self) {
        self.start_time = 0;
        self.elapsed = 0;
        self.state = NotStarted;
    }

    pub fn restart(&mut self) {
        self.reset();
        self.start();
    }
}
