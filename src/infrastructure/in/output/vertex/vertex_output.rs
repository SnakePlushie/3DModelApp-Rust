use bytemuck::NoUninit;

#[repr(C)]
#[derive(Copy, Clone, Debug, NoUninit)]
pub struct VertexOutput {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
