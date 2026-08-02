// windowing and stuff

use std::sync::Arc;

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
	render_pipeline: wgpu::RenderPipeline,
	vertex_buffer: wgpu::Buffer,
	indexx_buffer: wgpu::Buffer,
	staging_belt: wgpu::util::StagingBelt, // for performance
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
			format: wgpu::TextureFormat::Rgba8UnormSrgb,
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
		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: None,
			layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: None,
				bind_group_layouts: &[],
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
			label: None,
			size: 0x1000000,
			usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});
		let indexx_buffer = device.create_buffer(&wgpu::BufferDescriptor {
			label: None,
			size: 0x1000000,
			usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});
		let staging_belt = wgpu::util::StagingBelt::new(device.clone(), 0x40000);

		Self { window, surface, device, queue, config, render_pipeline, vertex_buffer, indexx_buffer, staging_belt }
	}

	fn resize(&mut self, width: u32, height: u32) {
		self.config.width = width;
		self.config.height = height;
		self.surface.configure(&self.device, &self.config);
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
			let indexx_slice = bytemuck::cast_slice(&context.draw_list.indices );
			self.staging_belt.write_buffer(&mut encoder, &self.vertex_buffer, 0, wgpu::BufferSize::new(vertex_slice.len() as u64).unwrap()).copy_from_slice(vertex_slice);
			self.staging_belt.write_buffer(&mut encoder, &self.indexx_buffer, 0, wgpu::BufferSize::new(indexx_slice.len() as u64).unwrap()).copy_from_slice(indexx_slice);
			self.staging_belt.finish();

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
			render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
			render_pass.set_index_buffer(self.indexx_buffer.slice(..), wgpu::IndexFormat::Uint16);
			render_pass.draw_indexed(0..context.draw_list.indices.len() as u32, 0, 0..1);
		}

		self.queue.submit(std::iter::once(encoder.finish()));
		self.staging_belt.recall();
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