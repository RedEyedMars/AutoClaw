extern crate gl;
extern crate sdl2;
extern crate vec_2_10_10_10;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate auto_claw_render_gl_derive as render_gl_derive;
extern crate rayon;

mod debug;

use failure::err_msg;

pub mod game;
pub mod gui;

use gui::render_gl::Viewport;
use gui::resources::Resources;

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path("assets").unwrap();

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let mut viewport = Viewport::for_window(900, 700);
    unsafe {
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut game = crate::game::GameLoop::new();
    let mut events = crate::game::Events::new();

    game.add_entity(&mut events, &res, &gl)?;

    // main loop

    use std::time::Instant;
    let now = Instant::now();
    let mut previous_millis = now.elapsed().as_millis();
    let millis_per_frame: u128 = 200; //17;
    let mut tick: u128 = 0;

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                }
                _ => {}
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangle

        //triangle.shift(0.0,1.0/8.0);
        let current_millis = now.elapsed().as_millis();
        tick += (current_millis - previous_millis) * 100000 / millis_per_frame;
        let ticks = tick / 100000;

        events = crate::game::GameLoop::execute(&mut game, &ticks, events);
        //e.render(&gl,&ticks);
        window.gl_swap_window();

        previous_millis = current_millis;
        tick -= ticks * 100000;

        //use std::{thread, time};
        //thread::sleep(time::Duration::from_millis(300));
    }
    Ok(())
}
