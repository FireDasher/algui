use algui::{Context, Rect, containers::Window, platform::{App, AppBuilder}, widgets::Button};
use glam::{Vec2, vec2};

fn main() {
	AppBuilder::new("My AlGui App").show(MyApp::default()).unwrap();
}

#[derive(Default)]
struct MyApp {
	counter: i32,
}

impl App for MyApp {
	fn update(&mut self, ctx: &mut Context) {
		Window::new("Title").show(ctx, |ui| {
			ui.label(format!("Counter: {}", self.counter));
			if ui.button("Hello, World!") {
				println!("clicked button");
				self.counter += 1;
			}
		});
	}
}