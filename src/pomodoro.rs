use std::time::Instant;
use std::time::Duration;
use std::fmt;
use super::webblock;

pub enum PomodoroPhase {
    Work,
    Free,
}

impl fmt::Display for PomodoroPhase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PomodoroPhase::Work => write!(f, "Work"),
            PomodoroPhase::Free => write!(f, "Free"),
        }
    }
}

pub struct Timer {
    work_duration: Duration,
    free_duration: Duration,
    finished_work_cycles: i32,
    finished_free_cycles: i32,
    pub current_phase: PomodoroPhase,
    pub timer_started: Option<Instant>,
}

impl Default for Timer {
    fn default() -> Self {
        let work_duration = Duration::from_secs(30 * 60);
        let free_duration = Duration::from_secs(10 * 60);
        Timer::new(work_duration, free_duration)
    }
}


impl Timer {
    pub fn new(work_duration: Duration, free_duration: Duration) -> Timer {
        let new_pomodoro = Timer {
            work_duration, free_duration,
            finished_work_cycles: 0, 
            finished_free_cycles: 0,
            current_phase: PomodoroPhase::Work,
            timer_started: None,
        };
        return new_pomodoro;
    }
    pub fn current_phase_duration(&self) -> Duration {
        match self.current_phase {
            PomodoroPhase::Work => self.work_duration,
            PomodoroPhase::Free => self.free_duration,
        }
    }
    pub fn start_phase(&mut self) {
        if !self.timer_started.is_none() { return; }
        self.timer_started = Some(Instant::now());
        match self.current_phase {
            PomodoroPhase::Work => { webblock::add_web_blocks(); },
            PomodoroPhase::Free => { webblock::rm_web_blocks(); },
        }
    }
    pub fn complete_phase(&mut self) {
        match self.current_phase {
            PomodoroPhase::Work => {
                self.finished_work_cycles+=1;
                self.current_phase = PomodoroPhase::Free;
            }
            PomodoroPhase::Free => {
                self.finished_free_cycles+=1;
                self.current_phase = PomodoroPhase::Work;
            }
        }
        self.timer_started = None;
    }
}