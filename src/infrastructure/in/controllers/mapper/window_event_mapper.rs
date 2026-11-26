use super::super::window_manager::WindowManager;
use super::key_mapper::KeyMapper;

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
            WindowEvent::CloseRequested => window_manager.request_close(),
            WindowEvent::Resized(size) => window_manager.resize(size.width, size.height),
            WindowEvent::RedrawRequested => self.redraw_request(window_manager),
            WindowEvent::KeyboardInput {
                device_id: _,
                event: _,
                is_synthetic: _,
            } => {
                if let Ok(false) = self.0.to_use_case(&event) {
                    let _ = self.0.to_window_action(&event, window_manager);
                }
            }
            _ => {}
        }
    }

    fn redraw_request(&self, window_manager: &mut WindowManager) {
        match window_manager.render() {
            Ok(_) => {}
            Err(e) => {
                log::error!("{e:?}");
                window_manager.request_close();
            }
        }
    }
}
