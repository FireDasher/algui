// windowing and stuff

use std::sync::Arc;

use glam::vec2;
use wgpu::util::DeviceExt;
use winit::{application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::{Window, WindowId}};

use crate::{context::Context, drawing::Vertex};

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
		let mut app = AlguiWinit { title: self.title.clone(), context: Context::default(), state, window_state: None  };
		event_loop.run_app(&mut app)
	}
}

struct WindowState {
	window: Arc<Window>,
	surface: wgpu::Surface<'static>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	uniform_buffer: wgpu::Buffer,
	uniform_bind_group: wgpu::BindGroup,
	render_pipeline: wgpu::RenderPipeline,
	vertex_buffer: wgpu::Buffer,
	index_buffer: wgpu::Buffer,
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
			format: wgpu::TextureFormat::Bgra8UnormSrgb, // for some reason colors uploaded to the GPU are in ABGR even if this is set to Rgba8UnormSrgb, but BGRA is guaranteed to be supported
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::AutoVsync,
			alpha_mode: wgpu::CompositeAlphaMode::Auto,
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
			color_space: wgpu::SurfaceColorSpace::Auto,
		};
		surface.configure(&device, &config);

		let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

		let size = window.inner_size();
		let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Uniform Buffer"),
			contents: bytemuck::cast_slice(&[vec2(2.0 / size.width as f32, -2.0 / size.height as f32), vec2(-1.0, 1.0)]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});
		let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("Uniform Bind Group Layout"),
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None
					},
					count: None,
				}
			]
		});
		let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("Uniform Bind Group"),
			layout: &uniform_bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: uniform_buffer.as_entire_binding(),
				}
			]
		});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: None,
			layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: None,
				bind_group_layouts: &[Some(&uniform_bind_group_layout)],
				immediate_size: 0,
			})),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: None,
				buffers: &[ Some(wgpu::VertexBufferLayout {
					array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
					step_mode: wgpu::VertexStepMode::Vertex,
					attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Unorm8x4],
				}) ],
				compilation_options: wgpu::PipelineCompilationOptions::default()
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: None,
				compilation_options: wgpu::PipelineCompilationOptions::default(),
				targets: &[Some(wgpu::ColorTargetState {
					format: config.format,
					blend: Some(wgpu::BlendState::ALPHA_BLENDING),
					write_mask: wgpu::ColorWrites::ALL
				})]
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: None,
				polygon_mode: wgpu::PolygonMode::Fill,
				unclipped_depth: false,
				conservative: false
			},
			depth_stencil: None,
			multisample: wgpu::MultisampleState::default(),
			multiview_mask: None,
			cache: None
		});

		let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Vertex Buffer"),
			size: 0x1000000,
			usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});
		let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Index Buffer"),
			size: 0x1000000,
			usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});

		Self { window, surface, device, queue, config, uniform_buffer, uniform_bind_group, render_pipeline, vertex_buffer, index_buffer }
	}

	fn resize(&mut self, width: u32, height: u32) {
		self.config.width = width;
		self.config.height = height;
		self.surface.configure(&self.device, &self.config);
		let size = self.window.inner_size();
		self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[vec2(2.0 / size.width as f32, -2.0 / size.height as f32), vec2(-1.0, 1.0)]));
	}

	fn render(&mut self, context: &Context) {
		let output = match self.surface.get_current_texture() {
			wgpu::CurrentSurfaceTexture::Success(texture) => texture,
			wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => { self.surface.configure(&self.device, &self.config); surface_texture },
			wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Validation | wgpu::CurrentSurfaceTexture::Lost => { return },
			wgpu::CurrentSurfaceTexture::Outdated => { self.surface.configure(&self.device, &self.config); return },
		};
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

		{
			// push mesh data
			let vertex_slice = bytemuck::cast_slice(&context.draw_list.vertices);
			let index_slice = bytemuck::cast_slice(&context.draw_list.indices );

			self.queue.write_buffer(&self.vertex_buffer, 0, vertex_slice);
			self.queue.write_buffer(&self.index_buffer, 0, index_slice);

			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: None,
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					depth_slice: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.0,
							g: 0.0,
							b: 0.0,
							a: 1.0,
						}),
						store: wgpu::StoreOp::Store,
					}
				})],
				depth_stencil_attachment: None,
				timestamp_writes: None,
				occlusion_query_set: None,
				multiview_mask: None,
			});
			render_pass.set_pipeline(&self.render_pipeline);
			render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
			render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
			render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
			render_pass.draw_indexed(0..context.draw_list.indices.len() as u32, 0, 0..1);
		}

		self.queue.submit(std::iter::once(encoder.finish()));
		self.queue.present(output);
	}
}

struct AlguiWinit<UserState: App> {
	title: String,
	context: Context,
	state: UserState,
	window_state: Option<WindowState>,
}

impl<UserState: App> ApplicationHandler for AlguiWinit<UserState> {
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
		let window = event_loop.create_window(Window::default_attributes().with_title(&self.title)).unwrap();
		let state = pollster::block_on(WindowState::new(Arc::new(window)));
		self.window_state = Some(state);
	}
	fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
		self.window_state = None; // drop everything
	}
	fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
		if let Some(state) = &mut self.window_state && window_id == state.window.id() {
			match event {
				WindowEvent::CloseRequested => event_loop.exit(),
				WindowEvent::RedrawRequested => {
					self.context.draw_list.clear();
					self.state.update(&mut self.context);
					state.render(&self.context);
				},
				WindowEvent::Resized(size) => state.resize(size.width, size.height),
				_ => (),
			}
		}
	}
}