use crate::r#in::{
    dto::vertex::{get_vertices_output::GetVerticesOutput, vertex_output::VertexOutput},
    presenters::presenter::Presenter,
};
use application::{
    dto::vertex::get_vertices_dto::GetVerticesDto,
    interfaces::r#in::presenter_core::Presenter as PresenterCore,
};
pub struct GetVerticesPresenter {
    output: Option<GetVerticesOutput>,
}

impl GetVerticesPresenter {
    pub fn new() -> Self {
        Self { output: None }
    }
}

impl PresenterCore<GetVerticesDto> for GetVerticesPresenter {
    fn present(&mut self, dto: GetVerticesDto) {
        self.output = Some(GetVerticesOutput {
            value: dto
                .vertices
                .into_iter()
                .map(|vertex| VertexOutput::from(vertex))
                .collect::<Vec<_>>(),
        });
    }
}

impl Presenter<GetVerticesDto, GetVerticesOutput> for GetVerticesPresenter {
    fn get_output(&self) -> Result<GetVerticesOutput, String> {
        self.output.clone().ok_or(String::from(
            "Did you call the use case ? Output wasn't set",
        ))
    }
}
