// windowing and stuff

use std::sync::Arc;

use winit::{application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::{Window, WindowId}};

use crate::context::Context;

pub trait App {
	fn update(&mut self, ctx: &mut Context);
}

pub struct AppBuilder {
	title: String,
	always_update: bool,
}

impl AppBuilder {
	pub fn new(title: impl Into<String>) -> AppBuilder {
		AppBuilder{ title: title.into(), always_update: false }
	}

	pub fn always_update(mut self) -> Self {
		self.always_update = true;
		self
	}

	pub fn show<UserState: App>(&self, state: UserState) -> Result<(), EventLoopError> {
		let event_loop = EventLoop::new()?;
		event_loop.set_control_flow(if self.always_update {ControlFlow::Poll} else {ControlFlow::Wait});
		let mut app = AlguiWinit { context: Context::default(), state, window: None  };
		event_loop.run_app(&mut app)
	}
}

struct WindowState {
	window: Arc<Window>,
	surface: wgpu::Surface<'static>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
}

impl WindowState {
	async fn new(window: Arc<Window>) -> Self {
		let size = window.inner_size();

		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::PRIMARY,
			flags: Default::default(),
			memory_budget_thresholds: Default::default(),
			backend_options: Default::default(),
			display: None,
		});

		let surface = instance.create_surface(window.clone()).unwrap();

		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
				apply_limit_buckets: true,
			}).await.unwrap();

		let (device, queue) = adapter
			.request_device(&wgpu::DeviceDescriptor {
				label: None,
				required_features: wgpu::Features::empty(),
				experimental_features: wgpu::ExperimentalFeatures::disabled(),
				required_limits: wgpu::Limits::default(),
				memory_hints: Default::default(),
				trace: wgpu::Trace::Off,
			})
			.await.unwrap();

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: wgpu::TextureFormat::Bgra8UnormSrgb,
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::AutoVsync,
			alpha_mode: wgpu::CompositeAlphaMode::Auto,
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
			color_space: wgpu::SurfaceColorSpace::Auto,
		};

		

		Ok(Self { window, surface, device, queue, config })
	}
}

struct AlguiWinit<UserState: App> {
	context: Context,
	state: UserState,
	window: Option<Window>,
}

impl<UserState: App> ApplicationHandler for AlguiWinit<UserState> {
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
		self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
	}
	fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
		if let Some(window) = &self.window && window.id() == window_id {
			match event {
				WindowEvent::CloseRequested => event_loop.exit(),
				WindowEvent::RedrawRequested => self.state.update(&mut self.context),
				_ => (),
			}
		}
	}
}