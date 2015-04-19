extern crate sdl2;
extern crate gl;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::keycode::KeyCode;


pub fn main() {
  let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();
  
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask, sdl2::video::GLProfile::GLCoreProfile as i32);
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 3);
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 3);
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDoubleBuffer, 1);
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDepthSize, 24);
  
  let window = match Window::new(&sdl_context, "game", WindowPos::PosCentered, WindowPos::PosCentered, 800, 600, OPENGL) {
    Ok(window) => window,
    Err(err) => panic!("Failed to create window: {}", err)
  };
  
  let context = window.gl_create_context().unwrap();
  gl::load_with(|s| unsafe {
    std::mem::transmute(sdl2::video::gl_get_proc_address(s))
  });
  
  let mut running = true;
  let mut event_pump = sdl_context.event_pump();

  while running {
    for event in event_pump.poll_iter() {
      use sdl2::event::Event;

      match event {
        Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
          running = false
        },
        _ => {}
      }
    }
    
    // The rest of the game loop goes here...
    unsafe {
      // Clear the screen to black
      gl::Viewport(0, 0, 800, 600);
      gl::ClearColor(0.0, 0.0, 0.9, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    window.gl_swap_window();
  }
}