use algui::{Context, Rect, containers::Window, platform::{App, AppBuilder}};
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
		// Window::new("Title").show(ctx, |ui| {
		// 	ui.label(format!("Counter: {}", self.counter));
		// 	if ui.button("Hello World").clicked {
		// 		println!("clicked button");
		// 		self.counter += 1;
		// 	}
		// });
		ctx.draw_list.add_rect(Rect::new(Vec2::splat(150.0), Vec2::splat(350.0)), 0xFF8000FF);
		ctx.draw_list.add_rect(Rect::new(Vec2::splat(100.0), Vec2::splat(300.0)), ctx.colors.button);
		ctx.draw_list.add_bottom_right_triangle(Rect::new(vec2(200.0, 10.0), vec2(210.0, 20.0)), ctx.colors.resize_grip);
		// println!("{:?}, {:?}", ctx.draw_list.vertices, ctx.draw_list.indices);
		// println!("I am updating!");
	}
}