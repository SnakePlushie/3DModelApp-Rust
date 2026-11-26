use super::super::window_manager::WindowManager;
use super::key_mapper::KeyMapper;

use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;

pub struct WindowEventMapper(KeyMapper);

impl WindowEventMapper {
    /// WindowEventMapper constructor
    pub fn new(key_mapper: KeyMapper) -> Self {
        Self(key_mapper)
    }

    /// Map WindowEvent to a window action
    pub fn to_window_action(&self, event: WindowEvent, window_manager: &mut WindowManager) {
        match event {
            WindowEvent::CloseRequested => self.close_request(window_manager),
            WindowEvent::Resized(size) => self.resize_request(window_manager, size),
            WindowEvent::RedrawRequested => self.redraw_request(window_manager),
            WindowEvent::KeyboardInput { .. } => self.keyboard_input(&event, window_manager),
            _ => {}
        }
    }

    /// RedrawRequested WindowEvent process
    fn close_request(&self, window_manager: &mut WindowManager) {
        window_manager.request_close();
    }

    /// Resized WindowEvent process
    fn resize_request(&self, window_manager: &mut WindowManager, size: PhysicalSize<u32>) {
        window_manager.resize(size.width, size.height)
    }

    /// RedrawRequested WindowEvent process
    fn redraw_request(&self, window_manager: &mut WindowManager) {
        match window_manager.render() {
            Ok(_) => {}
            Err(e) => {
                log::error!("{e:?}");
                window_manager.request_close();
            }
        }
    }

    /// KeyboardInput WindowEvent process
    fn keyboard_input(&self, event: &WindowEvent, window_manager: &mut WindowManager) {
        if let Ok(false) = self.0.to_use_case(&event) {
            let _ = self.0.to_window_action(&event, window_manager);
        }
    }
}
