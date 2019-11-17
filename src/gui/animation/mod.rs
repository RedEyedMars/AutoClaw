
use crate::gui::render_gl::data;


#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32,
    #[location = "1"]
    tex_coord: data::f32_f32
}


pub mod img;
pub mod texture;
pub mod animaton;
