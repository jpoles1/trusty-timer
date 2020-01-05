use std::time::Instant;
use std::time::Duration;
use std::fmt;
use tokio::time;

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
        self.poll().await;
    }
    pub async fn poll(&mut self) {
        if self.timer_started.is_none() { return; }
        let mut interval = time::interval(Duration::from_secs(1));
        let cycle_duration = self.current_phase_duration();
        while (Instant::now() - self.timer_started.unwrap()) < cycle_duration {
            interval.tick().await;
            println!("Timer still not completed!");
        }
        self.complete_phase().await;
    }
    async fn complete_phase(&mut self) {
        println!("Your {} phase has been completed!", self.current_phase);
        self.timer_started = None;
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
    }
}