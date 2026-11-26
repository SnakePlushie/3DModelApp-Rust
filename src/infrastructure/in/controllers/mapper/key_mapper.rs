use super::super::window_manager::WindowManager;

use log::error;
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct KeyMapper;

impl KeyMapper {
    /// Extract code and key state
    fn get_code_and_state(&self, event: &KeyEvent) -> Result<(KeyCode, ElementState), String> {
        if let KeyEvent {
            physical_key: PhysicalKey::Code(code),
            state: key_state,
            ..
        } = event
        {
            return Ok((code.clone(), key_state.clone()));
        }
        let error = "App Error: Event is not processable";

        error!("{error}");
        Err(String::from(error))
    }

    /// Map KeyEvent to window action
    pub fn to_window_action(
        &self,
        event: &KeyEvent,
        window_manager: &mut WindowManager,
    ) -> Result<bool, String> {
        let (code, key_state) = self.get_code_and_state(event)?;

        match (code, key_state.is_pressed()) {
            (KeyCode::Escape, true) => {
                window_manager.request_close();
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Map KeyEvent to use case
    pub fn to_use_case(&self, event: &KeyEvent) -> Result<bool, String> {
        let (code, key_state) = self.get_code_and_state(event)?;
        match (code, key_state.is_pressed()) {
            _ => Ok(false),
        }
    }
}
