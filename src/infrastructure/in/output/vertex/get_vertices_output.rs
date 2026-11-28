use crate::r#in::output::vertex::vertex_output::VertexOutput;

#[derive(Clone, Debug)]
pub struct GetVerticesOutput {
    pub value: Vec<VertexOutput>,
}
