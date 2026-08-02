use algui::{Context, containers::Window, platform::{App, AppBuilder}};

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
			if ui.button("Hello World").clicked {
				println!("clicked button");
				self.counter += 1;
			}
		});
		println!("I am updating!");
	}
}