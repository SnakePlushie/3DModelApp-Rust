use std::sync::Arc;
use wgpu::RequestAdapterError;
use wgpu::RequestDeviceError;
use winit::window::Window;

pub struct WindowManager {
    window: Arc<Window>,
    close_requested: bool,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    is_surface_configured: bool,
}

#[derive(Debug)]
pub enum InitError {
    Adapter(RequestAdapterError),
    Device(RequestDeviceError),
}

impl From<RequestAdapterError> for InitError {
    fn from(err: RequestAdapterError) -> Self {
        Self::Adapter(err)
    }
}

impl From<RequestDeviceError> for InitError {
    fn from(err: RequestDeviceError) -> Self {
        Self::Device(err)
    }
}

impl WindowManager {
    /// WindowManager constructor
    pub async fn new(window: Arc<Window>) -> Result<Self, InitError> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
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
            })
            .await?;

        let (device, _queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        Ok(Self {
            window: window,
            close_requested: false,
            config,
            surface,
            device,
            is_surface_configured: false,
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
    pub fn render(&mut self) {
        self.window.request_redraw();
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
}
