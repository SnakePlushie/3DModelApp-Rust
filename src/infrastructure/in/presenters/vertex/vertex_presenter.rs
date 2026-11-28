use domain::entities::vertex::Vertex;

use crate::r#in::output::vertex::vertex_output::VertexOutput;

impl From<Vertex> for VertexOutput {
    fn from(vertex: Vertex) -> Self {
        Self {
            position: [
                vertex.get_position().x,
                vertex.get_position().y,
                vertex.get_position().z,
            ],
            color: [
                vertex.get_color().r,
                vertex.get_color().g,
                vertex.get_color().b,
            ],
        }
    }
}
