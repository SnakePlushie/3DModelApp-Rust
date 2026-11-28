use domain::{
    entities::vertex::Vertex,
    value_objects::{color::Color, position::Position},
};

use crate::{
    dto::vertex::get_vertices_dto::GetVerticesDto,
    interfaces::r#in::{
        presenter_core::Presenter, vertex::get_vertices_interface::GetVerticesInterface,
    },
};

#[derive(Clone, Copy, Debug)]
pub struct GetVerticesUseCase;

impl GetVerticesInterface for GetVerticesUseCase {
    fn execute(&self, presenter: &mut impl Presenter<GetVerticesDto>) {
        let vertices: Vec<Vertex> = vec![
            Vertex::new(
                Position {
                    x: 0.0,
                    y: 0.5,
                    z: 0.0,
                },
                Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                },
            ),
            Vertex::new(
                Position {
                    x: -0.5,
                    y: -0.5,
                    z: 0.0,
                },
                Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                },
            ),
            Vertex::new(
                Position {
                    x: 0.5,
                    y: -0.5,
                    z: 0.0,
                },
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 1.0,
                },
            ),
        ];

        presenter.present(GetVerticesDto { vertices })
    }
}
