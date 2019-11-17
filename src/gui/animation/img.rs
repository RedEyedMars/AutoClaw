use failure;
use gl;
use crate::gui::render_gl::{self, buffer};
use crate::gui::resources::{Resources};

use crate::gui::animation::{Vertex,texture,animaton};

pub struct Img {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
    animaton: animaton::Animaton,
}

impl Img {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Img, failure::Error> {
        let program = render_gl::Program::from_res(gl, res, "shaders/dynamic_img")?;
        let tex = texture::Texture::create_texture(&res,&gl)?;
        let animaton_ = animaton::Animaton::new(tex,animaton::AnimationTypes::X_SHIFT_4_x_8,program.get_tex_coord_shift_loc());
        let initial_tex_coord = animaton_.get_initial_tex_coord();
        let vertices: Vec<Vertex> = vec![
        Vertex { pos: (0.0, 0.0).into(), tex_coord: initial_tex_coord.0,  }, // bottom left
        Vertex { pos: (0.0, 0.5).into(), tex_coord: initial_tex_coord.1, }, //top left
        Vertex { pos: (0.5, 0.0).into(), tex_coord: initial_tex_coord.2, },//bottom right
        Vertex { pos: (0.0, 0.5).into(), tex_coord: initial_tex_coord.3, },//bottom right
        Vertex { pos: (0.5, 0.5).into(), tex_coord: initial_tex_coord.4,  },//bottom right
        Vertex { pos: (0.5, 0.0).into(), tex_coord: initial_tex_coord.5, },//bottom right
        ];

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        // set up vertex array object

        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Img {
            program: program,
            _vbo: vbo,
            vao,
            animaton: animaton_,
        })
    }
/*
    pub fn shift(&mut self,x:f32,y:f32){

        self.x_shift += x;
        self.y_shift += y;
    }
*/
    pub fn render(&mut self, gl: &gl::Gl, ticks :&u128) {
        self.program.set_used();
        self.animaton.animate(&gl,&ticks);
        self.vao.bind();
        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                6,             // number of indices to be rendered
            );
        }
    }
}
