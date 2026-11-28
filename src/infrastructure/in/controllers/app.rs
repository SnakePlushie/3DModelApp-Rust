use application::use_cases::vertex::get_vertices::GetVerticesUseCase;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;

use super::mapper::window_event_mapper::WindowEventMapper;
use super::window_manager::WindowManager;

pub struct App {
    window_manager: Option<WindowManager>,
    window_event_mapper: WindowEventMapper,
    vertex_service: GetVerticesUseCase,
}

impl App {
    /// WindowEventMapper constructor
    pub fn new(window_event_mapper: WindowEventMapper, vertex_service: GetVerticesUseCase) -> Self {
        Self {
            window_manager: None,
            window_event_mapper,
            vertex_service,
        }
    }

    /// Getter for the verte service
    pub fn get_vertex_service(&self) -> GetVerticesUseCase {
        self.vertex_service
    }

    /// Setter for a window manager
    pub fn set_window_manager(&mut self, window_manager: Option<WindowManager>) {
        self.window_manager = window_manager;
    }

    /// Process a window event
    pub fn process_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        let window_manager: &mut WindowManager = match &mut self.window_manager {
            Some(canvas) => canvas,
            None => return,
        };

        if window_manager.get_close_requested() {
            event_loop.exit();
            return;
        }

        self.window_event_mapper
            .to_window_action(event, window_manager);
    }
}
