use orbtk::prelude::*;
mod small_btn;
mod numeric_text_box;

#[derive(Debug, Copy, Clone)]
enum NumericInputAction {
	Increment,
	Decrement,
}

#[derive(Default, AsAny)]
pub struct NumericInputState {
    action: Option<NumericInputAction>,
}


impl NumericInputState {
    fn action(&mut self, action: impl Into<Option<NumericInputAction>>) {
        self.action = action.into();
    }
}

impl State for NumericInputState {
	fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
		if let Some(action) = self.action {
			match action {
				NumericInputAction::Increment => {
					let increment = ctx.widget().clone::<f32>("increment");
					let current_val: f32 = ctx.child("numeric-text-box").get::<f32>("value") + increment;
					ctx.widget().set("value", current_val);
					ctx.child("numeric-text-box").set("value", current_val);
					ctx.child("numeric-text-box").set("text", String16::from(format!("{}", current_val)));
				}
				NumericInputAction::Decrement => {
					let increment = ctx.widget().clone::<f32>("increment");
					let current_val: f32 = ctx.child("numeric-text-box").get::<f32>("value") - increment;
					ctx.widget().set("value", current_val);
					ctx.child("numeric-text-box").set("value", current_val);
					ctx.child("numeric-text-box").set("text", String16::from(format!("{}", current_val)));
				}
			}
			self.action = None;
		}
	}
}

widget!(NumericInput<NumericInputState> {
	// Sets or shares the increment property.
	value: f32,
	// Sets or shares the increment property.
	increment: f32
});

// helper to request NumericInputState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut NumericInputState {
    states.get_mut(id)
}

impl Template for NumericInput {
	fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
		self.name("NumericInput")
			.child(
				Container::create().size(80.0, 40.0)
				.border_width(2.0).border_brush(Brush::SolidColor(Color::rgb(50, 50, 50)))
				.child(
					Grid::create().columns(Columns::create().column(30.0).column("*").build())
					.child(
						Grid::create().rows(Rows::create().row(20.0).row(20.0).build())
						.child(
							small_btn::SmallButton::create().text("+")
							.border_width(2.0).border_brush(Brush::SolidColor(Color::rgb(50, 50, 50)))
							.on_click(move |states, _| -> bool {
								state(id, states).action(NumericInputAction::Increment);
								true
							}).attach(Grid::row(0)).build(ctx)
						).child(
							small_btn::SmallButton::create().text("-")
							.border_width(2.0).border_brush(Brush::SolidColor(Color::rgb(50, 50, 50)))
							.on_click(move |states, _| -> bool {
								state(id, states).action(NumericInputAction::Decrement);
								true
							}).attach(Grid::row(1)).build(ctx)
						).attach(Grid::column(0)).build(ctx)
					).child(
						numeric_text_box::NumericTextBox::create()
						.value(id).increment(id)
						.selector(Selector::from("numeric-text-box").id("numeric-text-box"))
						.vertical_alignment("center").horizontal_alignment("start")
						.attach(Grid::column(1)).build(ctx)
					).build(ctx)
				).build(ctx)
			)
	}
}