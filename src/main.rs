use std::time::Duration;
use std::io::{stdin, stdout, Read, Write};
use orbtk::prelude::*;

mod pomodoro;
mod sounds;
mod webblock; 

static WIN_WIDTH: f64 = 400.0;
static WIN_HEIGHT: f64 = 200.0;

fn term_pause(msg: String) {
    let mut stdout = stdout();
    stdout.write(msg.as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0;2]).unwrap();
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Start,
}

#[derive(Default)]
pub struct MainViewState {
    blocklist: String,
    timer: pomodoro::Timer,
    action: Option<Action>,
}


impl MainViewState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

impl State for MainViewState {
    fn update(&self, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Start => {
                }
            }
            //self.action = None;
        }
    }
}

widget!(MainView<MainViewState> {
    text: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        return self.name("Main View").child(
            TextBlock::create().text("Trusty Timer").horizontal_alignment("Center").font_size(20.0).build(ctx)
        );
    }
}

fn start_gui() {
    Application::new()
    .window(|ctx| {
        Window::create()
            .title("Trusty Timer")
            .position((100.0, 100.0))
            .size(WIN_WIDTH, WIN_HEIGHT)
            .child(MainView::create().build(ctx))
            .build(ctx)
    })
    .run();
}

#[tokio::main]
async fn main() {
    start_gui();
    webblock::rm_web_blocks();
    let mut timer = pomodoro::Timer::new(Duration::from_secs(25 * 60), Duration::from_secs(5 * 60));
    loop {
        sounds::play_ding();
        term_pause(format!("\nPress enter to start your next phase: {} for {} minutes\n", timer.current_phase, timer.current_phase_duration().as_secs()/60));
        timer.start_phase().await;
    }
}