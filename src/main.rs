#![windows_subsystem = "windows"]

use std::time::Instant;
use orbtk::{prelude::*, shell::ShellRequest};

mod pomodoro;
mod sounds;
mod webblock; 

static WIN_WIDTH: f64 = 320.0;
static WIN_HEIGHT: f64 = 160.0;

#[derive(Debug, Copy, Clone)]
enum Action {
    Start,
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    timer: pomodoro::Timer,
    action: Option<Action>,
    last_update: Option<Instant>,
}


impl MainViewState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

impl State for MainViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        ctx.widget().set("timer_not_started", true);
        ctx.widget().set("phase_string", String16::from("Current Phase: Work"));
        ctx.widget().set("time_remaining_string", String16::from("Time Remaining: Not Started"));
    }
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::Start => {
                    ctx.widget().set("timer_not_started", false);
                    self.timer.start_phase();
                }
            }
            self.action = None;
        }
        if !self.timer.timer_started.is_none() {
            let cycle_duration = self.timer.current_phase_duration();
            if (Instant::now() - self.timer.timer_started.unwrap()) > cycle_duration {
                self.timer.complete_phase();
                ctx.widget().set("phase_string", String16::from(format!("Current Phase: {}", self.timer.current_phase)));
                ctx.widget().set("timer_not_started", true);
                ctx.request_sender().send(ShellRequest::Update).unwrap();
                sounds::play_ding();
            } else {
                let time_remaining = self.timer.timer_started.unwrap() + cycle_duration - Instant::now();
                ctx.widget().set("time_remaining_string", String16::from(format!("Time Remaining: {} sec",  time_remaining.as_secs() )));
                if self.last_update.is_none() || (Instant::now() - self.last_update.unwrap()).as_millis() > 500 {
                    ctx.request_sender().send(ShellRequest::Update).unwrap();    
                    self.last_update = Some(Instant::now());
                }
            }
        } else {
            ctx.widget().set("time_remaining_string", String16::from("Time Remaining: Not Started"));
        }
    }
}

widget!(MainView<MainViewState> {
    phase_string: String16,
    timer_not_started: bool,
    time_remaining_string: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        return self.name("Main View").child(
            Grid::create().rows(Rows::create().row("*").row("*").row("*").build())
            .child(
                TextBlock::create().text("Trusty Timer").horizontal_alignment("Center").font_size(20.0).attach(Grid::row(0)).build(ctx)
            ).child(
                TextBlock::create().text(("phase_string", id)).horizontal_alignment("Center").font_size(16.0).attach(Grid::row(1)).build(ctx)
            ).child(
                TextBlock::create().text(("time_remaining_string", id)).horizontal_alignment("Center").margin((0, 20, 0, 0)).font_size(16.0).attach(Grid::row(1)).build(ctx)
            ).child(
                Button::create().text("Start Timer").attach(Grid::row(2))
                .enabled(("timer_not_started", id))
                .on_click(move |states, _| -> bool {
                    state(id, states).action(Action::Start);
                    true
                }).build(ctx)
            ).build(ctx)
        )
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
    webblock::rm_web_blocks();
    start_gui();
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}