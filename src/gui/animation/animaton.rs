use gl;

use crate::gui::render_gl::data;

use crate::gui::animation::{texture};
pub struct AnimationType(u128,f32,f32);
#[allow(non_snake_case)]
pub mod AnimationTypes {
    use crate::gui::animation::animaton::AnimationType;

    #[allow(non_upper_case_globals)]
    pub const X_SHIFT_4_x_8:AnimationType = AnimationType(4u128,4.0f32,8.0f32);
}
pub struct Animaton {
    tex: texture::Texture,
    x_shift: f32,
    y_shift: f32,
    direction: i8,
    ani_type: AnimationType,
    tex_coord_loc: gl::types::GLint,
}

impl Animaton {
    pub fn new<'a>(tex: texture::Texture, ani_type:AnimationType, loc:gl::types::GLint) -> Animaton {
        println!("{}",(ani_type.2 - 1.0)/ani_type.2);
        return Animaton{
            tex: tex,
            x_shift:0.0,
            y_shift:(ani_type.2 - 1.0)/ani_type.2,
            direction:1,
            ani_type: ani_type,
            tex_coord_loc: loc,
        }
    }
    pub fn shift(&mut self, y_shift:u16){
        self.y_shift = 1.0-((y_shift as f32)+1.0)/self.ani_type.2;
    }

    pub fn get_initial_tex_coord(&self) -> ((data::f32_f32),(data::f32_f32),(data::f32_f32),(data::f32_f32),(data::f32_f32),(data::f32_f32)) {
        return (
            (0.0,0.0).into(),
            (0.0,1.0/self.ani_type.2).into(),
            (1.0/self.ani_type.1,0.0).into(),
            (0.0,1.0/self.ani_type.2).into(),
            (1.0/self.ani_type.1,1.0/self.ani_type.2).into(),
            (1.0/self.ani_type.1,0.0).into(),
        );
    }
    pub fn animate(&mut self,gl :&gl::Gl, ticks: &u128){
        if *ticks != 0 as u128 {
            if self.direction > 0 {
                self.x_shift = self.x_shift+(*ticks as f32)/self.ani_type.1;
                if self.x_shift == 1.0 {
                    self.x_shift = 1.0-2.0/self.ani_type.1;
                    self.direction = -1;
                } else if self.x_shift>1.0 {
                    let ones = ((self.x_shift*10.0) as u32)/10u32;
                    if ones%2==1 {
                        self.x_shift = 1.00-(self.x_shift-(ones as f32))-1.0/self.ani_type.1;
                        self.direction = -1;
                    }
                    else {
                        self.x_shift = self.x_shift-(ones as f32);
                        self.direction = 1;
                    }
                }
            } else {
                if self.x_shift == 0.0 {
                    self.x_shift = self.x_shift+(*ticks as f32)/self.ani_type.1;
                    let ones = ((self.x_shift*10.0) as u32)/10u32;
                    if ones==0u32 {
                        self.direction = 1;
                    } else if ones%2==1 {
                        self.x_shift = 1.00-(self.x_shift-(ones as f32));
                        self.direction = -1;
                    }
                    else {
                        self.x_shift = self.x_shift-(ones as f32);
                        self.direction = 1;
                    }
                } else {
                    self.x_shift = self.x_shift-(*ticks as f32)/self.ani_type.1;
                    if self.x_shift <= 0.0 {
                        let ones = ((self.x_shift*-10.0) as u32)/10u32;
                        if ones%2==1 {
                            self.x_shift = 1.00-(-self.x_shift-(ones as f32));
                            self.direction = -1;
                        }
                        else {
                            self.x_shift = -self.x_shift-(ones as f32);
                            self.direction = 1;
                        }
                    }
                }
            }
            //println!("{}",self.x_shift);
        }
        unsafe {
            gl.ActiveTexture(gl::TEXTURE0);
        }
        self.tex.bind(gl);
        unsafe {
            gl.Uniform2f(self.tex_coord_loc, self.x_shift, self.y_shift);
        }
    }
}
