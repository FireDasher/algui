use algui::{Context, platform::{App, AppBuilder}};

fn main() {
	AppBuilder::new("My AlGui App").show(MyApp::default()).unwrap();
}

#[derive(Default)]
struct MyApp {
	counter: i32,
}

impl App for MyApp {
	fn update(&mut self, ctx: &mut Context) {
		ctx.window("Example Window", |ui| {
			ui.label(&format!("Counter: {}", self.counter));
			if ui.button("Hello, World!").clicked {
				println!("clicked button");
				self.counter += 1;
			}
			ui.same_line();
			if ui.button("Goodbye, Moon!").clicked {
				println!("clicked EVIL button");
				self.counter -= 1;
			}
			ui.same_line();
			ui.button("Other Button");
			ui.button("Not a button!");
		});
	}
}