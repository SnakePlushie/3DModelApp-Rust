use super::window_manager::WindowManager;

pub struct App(Option<WindowManager>);

impl App {
    /// WindowEventMapper constructor
    pub fn new() -> Self {
        Self(None)
    }

    /// Getter for a window manager
    pub fn get_window_manager(&mut self) -> Option<&mut WindowManager> {
        self.0.as_mut()
    }

    /// Setter for a window manager
    pub fn set_window_manager(&mut self, window_manager: Option<WindowManager>) {
        self.0 = window_manager;
    }
}
