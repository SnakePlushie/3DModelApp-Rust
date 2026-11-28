use crate::value_objects::{color::Color, position::Position};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    position: Position,
    color: Color,
}

impl Vertex {
    /// Vertex constructor
    pub fn new(position: Position, color: Color) -> Self {
        Self { position, color }
    }

    /// Getter for the position
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Getter for the color
    pub fn get_color(&self) -> &Color {
        &self.color
    }

    /// Setter for the position
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Setter for the color
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

#[cfg(test)]
#[path = "../test/unit/entities/vertex_test.rs"]
mod tests;
