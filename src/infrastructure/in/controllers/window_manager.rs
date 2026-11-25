use std::sync::Arc;
use winit::window::Window;

pub struct WindowManager {
    window: Arc<Window>,
    close_requested: bool,
}

impl WindowManager {
    /// WindowManager constructor
    pub async fn new(window: Arc<Window>) -> Result<Self, ()> {
        Ok(Self {
            window: window,
            close_requested: false,
        })
    }

    /// Resize the window
    pub fn resize(&mut self, _width: u32, _height: u32) {}

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
