use std::time::Duration;
use std::io::{stdin, stdout, Read, Write};

mod pomodoro;

fn term_pause(msg: String) {
    let mut stdout = stdout();
    stdout.write(msg.as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[tokio::main]
async fn main() {
    let mut timer = pomodoro::Timer::new(Duration::from_secs(60 * 25), Duration::from_secs(60 * 5));
    loop {
        term_pause(format!("Press enter to start your next phase: {} for {} minutes", timer.current_phase, timer.current_phase_duration().as_secs()/60));
        timer.start_phase().await;
    }
}