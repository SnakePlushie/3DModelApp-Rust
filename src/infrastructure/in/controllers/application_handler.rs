use super::app::App;
use super::window_manager::WindowManager;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler, event::*, event_loop::ActiveEventLoop, window::Window,
};

impl ApplicationHandler<WindowManager> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // We use pollster to await the window creation
        self.set_window_manager(Some(
            pollster::block_on(WindowManager::new(window, self.get_vertex_service())).unwrap(),
        ));
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut new_window_manager: WindowManager) {
        self.set_window_manager(Some(new_window_manager));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.process_window_event(event_loop, event);
    }
}
