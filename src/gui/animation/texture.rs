use gl;
use crate::gui::resources::{error, Resources};


pub struct Texture {
    tex: gl::types::GLuint,
}

impl Texture {
    pub fn bind(&self, gl: &gl::Gl){
        unsafe { gl.BindTexture(gl::TEXTURE_2D, self.tex); }
    }

    pub fn create_texture( res: &Resources, gl: &gl::Gl) -> Result<Texture,error::Error> {
        let mut tex: gl::types::GLuint = 0;
        unsafe {
            gl.GenTextures(1, &mut tex);
            gl.BindTexture(gl::TEXTURE_2D, tex);
        }

        let img = res.load_rgba_image("textures/cat.png".to_string())?;

        unsafe {

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as gl::types::GLint,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const ::std::os::raw::c_void,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }

        return Ok(Texture {
            tex,
        });

    }

}
