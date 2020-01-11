use std::time::Instant;
use std::time::Duration;
use std::fmt;
use std::io::Stdout;
use tokio::time;
use pbr::ProgressBar;
use super::webblock;

pub enum PomodoroPhase {
    Work,
    Free,
}

impl fmt::Display for PomodoroPhase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PomodoroPhase::Work => write!(f, "work"),
            PomodoroPhase::Free => write!(f, "free"),
        }
    }
}

pub struct Timer {
    work_duration: Duration,
    free_duration: Duration,
    finished_work_cycles: i32,
    finished_free_cycles: i32,
    pub current_phase: PomodoroPhase,
    timer_started: Option<Instant>,
    pbar: ProgressBar<Stdout>,
}

impl Default for Timer {
    fn default() -> Self {
        let work_duration = Duration::from_secs(25 * 60);
        let free_duration = Duration::from_secs(5 * 60);
        return Timer {
            work_duration, free_duration,
            finished_work_cycles: 0, 
            finished_free_cycles: 0,
            current_phase: PomodoroPhase::Work,
            timer_started: None,
            pbar: ProgressBar::new(work_duration.as_secs()),
        };
    }
}


impl Timer {
    pub fn new(work_duration: Duration, free_duration: Duration) -> Timer {
        let mut new_pomodoro = Timer {
            work_duration, free_duration,
            finished_work_cycles: 0, 
            finished_free_cycles: 0,
            current_phase: PomodoroPhase::Work,
            timer_started: None,
            pbar: ProgressBar::new(work_duration.as_secs()),
        };
        new_pomodoro.pbar.show_speed = false;
        new_pomodoro.pbar.show_time_left = false;
        println!("Starting your Pomodoro session, get ready to be productive!");
        return new_pomodoro;
    }
    pub fn current_phase_duration(&self) -> Duration {
        match self.current_phase {
            PomodoroPhase::Work => self.work_duration,
            PomodoroPhase::Free => self.free_duration,
        }
    }
    pub async fn start_phase(&mut self) {
        if !self.timer_started.is_none() { return; }
        self.timer_started = Some(Instant::now());
        self.pbar.set(0);
        self.pbar.total = self.current_phase_duration().as_secs();
        match self.current_phase {
            PomodoroPhase::Work => { webblock::add_web_blocks(); },
            PomodoroPhase::Free => { webblock::rm_web_blocks(); },
        }
        self.poll().await;
    }
    pub async fn poll(&mut self) {
        if self.timer_started.is_none() { return; }
        let mut interval = time::interval(Duration::from_secs(5));
        let cycle_duration = self.current_phase_duration();
        while (Instant::now() - self.timer_started.unwrap()) < cycle_duration {
            interval.tick().await;
            self.pbar.set((Instant::now() - self.timer_started.unwrap()).as_secs());
        }
        self.complete_phase();
    }
    fn complete_phase(&mut self) {
        println!("\nYour {} phase has been completed!", self.current_phase);
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