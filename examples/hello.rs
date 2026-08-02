use algui::{Context, Rect, containers::Window, platform::{App, AppBuilder}};
use glam::Vec2;

fn main() {
	AppBuilder::new("My AlGui App").show(MyApp::default()).unwrap();
}

#[derive(Default)]
struct MyApp {
	counter: i32,
}

impl App for MyApp {
	fn update(&mut self, ctx: &mut Context) {
		// Window::new("Title").show(ctx, |ui| {
		// 	ui.label(format!("Counter: {}", self.counter));
		// 	if ui.button("Hello World").clicked {
		// 		println!("clicked button");
		// 		self.counter += 1;
		// 	}
		// });
		ctx.draw_list.add_rect(Rect::new(Vec2::splat(100.0), Vec2::splat(300.0)), 0xFF8000FF);
		ctx.draw_list.add_rect(Rect::new(Vec2::splat(150.0), Vec2::splat(350.0)), 0x0000FFFF);
		println!("I am updating!");
	}
}