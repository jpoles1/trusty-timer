use std::time::Duration;
use std::io::{stdin, stdout, Read, Write};

mod pomodoro;

fn term_pause(msg: String) {
    let mut stdout = stdout();
    stdout.write(msg.as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0;2]).unwrap();
}

#[tokio::main]
async fn main() {
    let mut timer = pomodoro::Timer::new(Duration::from_secs(25 * 60), Duration::from_secs(5 * 60));
    loop {
        term_pause(format!("\nPress enter to start your next phase: {} for {} minutes\n", timer.current_phase, timer.current_phase_duration().as_secs()/60));
        timer.start_phase().await;
    }
}