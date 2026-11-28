use application::{
    interfaces::r#in::vertex::get_vertices_interface::GetVerticesInterface,
    use_cases::vertex::get_vertices::GetVerticesUseCase,
};
use log::info;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::r#in::presenters::{
    presenter::Presenter, vertex::get_vertices_presenter::GetVerticesPresenter,
};

pub struct WindowManager {
    window: Arc<Window>,
    close_requested: bool,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_service: GetVerticesUseCase,
}

#[derive(Debug)]
pub enum InitError {
    Adapter(wgpu::RequestAdapterError),
    Device(wgpu::RequestDeviceError),
    Vertices(String),
}

impl From<wgpu::RequestAdapterError> for InitError {
    fn from(err: wgpu::RequestAdapterError) -> Self {
        Self::Adapter(err)
    }
}

impl From<wgpu::RequestDeviceError> for InitError {
    fn from(err: wgpu::RequestDeviceError) -> Self {
        Self::Device(err)
    }
}

impl From<String> for InitError {
    fn from(err: String) -> Self {
        Self::Vertices(err)
    }
}

impl WindowManager {
    /// WindowManager constructor
    pub async fn new(
        window: Arc<Window>,
        vertex_service: GetVerticesUseCase,
    ) -> Result<Self, InitError> {
        // Handle to our GPU / instance of WGPU
        let instance = Self::get_wgpu_instance();

        // The part of the window we can draw to
        let surface = instance.create_surface(window.clone()).unwrap();

        // Handle of Graphic card, get info about it
        let adapter = Self::get_gpu_adapter(&instance, &surface).await?;

        // device / rendering queue
        let (device, queue) = Self::get_device_and_queue(&adapter).await?;

        let config = Self::get_config(&window, &surface, &adapter);

        let render_pipeline = Self::get_render_pipeline(&device, &config);

        let mut presenter = GetVerticesPresenter::new();

        vertex_service.execute(&mut presenter);
        let output = presenter.get_output()?.value;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(output.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Ok(Self {
            window: window,
            close_requested: false,
            config,
            surface,
            device,
            queue,
            is_surface_configured: false,
            render_pipeline,
            vertex_buffer,
            vertex_service,
        })
    }

    /// Resize the window
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    /// Redraw the window
    pub fn render(&mut self) -> Result<(), ()> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            info!("Render: Surface was not configured");
            return Ok(());
        }

        // We render to a SurfaceTexture
        let surface_texture = match self.get_surface_texture() {
            Some(surface_texture) => surface_texture,
            None => {
                info!("Render: Coudln't fetch surface");
                return Ok(());
            }
        };

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Map command to gpu understandable
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Has method for the drawing
        self.set_render_pass(&mut encoder, &view);

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(surface_texture);

        Ok(())
    }

    /// Request close the window
    pub fn request_close(&mut self) {
        self.close_requested = true;
        self.window.request_redraw();
    }

    /// Getter for close request
    pub fn get_close_requested(&self) -> bool {
        return self.close_requested;
    }

    /// Getter for WGPU instance
    fn get_wgpu_instance() -> wgpu::Instance {
        wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY, // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        })
    }

    /// Getter for gpu adapter
    async fn get_gpu_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface<'_>,
    ) -> Result<wgpu::Adapter, wgpu::RequestAdapterError> {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
                apply_limit_buckets: true,
            })
            .await
    }

    /// Getter for the device and rendering queue
    async fn get_device_and_queue(
        adapter: &wgpu::Adapter,
    ) -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError> {
        adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
    }

    /// Getter for the window config
    fn get_config(
        window: &Arc<Window>,
        surface: &wgpu::Surface,
        adapter: &wgpu::Adapter,
    ) -> wgpu::SurfaceConfiguration {
        let size: winit::dpi::PhysicalSize<u32> = window.inner_size();

        let surface_capabilities = surface.get_capabilities(&adapter);

        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);

        // Config of the window
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        }
    }

    /// Getter for the render pipeline
    fn get_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../shader/shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            // vertex shader
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // 1.
                buffers: &[],                 // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            // fragment shader
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview_mask: None, // 5.
            cache: None,          // 6.
        })
    }

    /// Getter for the current surface texture
    fn get_surface_texture(&self) -> Option<wgpu::SurfaceTexture> {
        match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => Some(surface_texture),
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => Some(surface_texture),
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                // Skip this frame
                return None;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return None;
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                // You could recreate the devices and all resources
                // created with it here, but we'll just bail
                info!("Lost device");
                return None;
            }
        }
    }

    /// Do the render pass
    fn set_render_pass(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    // background
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.05,
                        b: 0.25,
                        a: 0.4,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        });

        render_pass.set_pipeline(&self.render_pipeline); // 2.
        render_pass.draw(0..3, 0..1); // 3.
    }
}
