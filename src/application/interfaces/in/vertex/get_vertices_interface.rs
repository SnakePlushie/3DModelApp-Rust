use crate::{
    dto::vertex::get_vertices_dto::GetVerticesDto, interfaces::r#in::presenter_core::Presenter,
};

pub trait GetVerticesInterface {
    fn execute(&self, presenter: &mut impl Presenter<GetVerticesDto>);
}
