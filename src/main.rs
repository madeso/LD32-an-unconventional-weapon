extern crate sdl2;
extern crate glium;
extern crate libc;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::keycode::KeyCode;
use std::rc::Rc;
use glium::Surface;

pub fn main() {
  let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();
  
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask, sdl2::video::GLProfile::GLCoreProfile as i32);
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 3);
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 3);
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDoubleBuffer, 1);
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDepthSize, 24);
  
  let mut window = match Window::new(&sdl_context, "game", WindowPos::PosCentered, WindowPos::PosCentered, 800, 600, OPENGL) {
    Ok(window) => window,
    Err(err) => panic!("Failed to create window: {}", err)
  };
  let context = window.gl_create_context().unwrap();
  let mut event_pump = sdl_context.event_pump();
  let dsize = window.properties(&event_pump).get_drawable_size();
  let rcwindow = Rc::new(window);
  
  struct Backend {
    window: Rc<Window>,
    size : (i32, i32),
  }
  unsafe impl glium::backend::Backend for Backend {
    fn swap_buffers(&self) {
        self.window.gl_swap_window();
    }
    unsafe fn get_proc_address(&self, symbol: &str) -> *const libc::c_void {
      std::mem::transmute(sdl2::video::gl_get_proc_address(symbol))
    }
    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
      let size = self.size;
      let (w,h) = size;
      (w as u32, h as u32)
    }
    fn is_current(&self) -> bool {
      false
    }
    unsafe fn make_current(&self) {
      let gl_context = match sdl2::video::gl_get_current_context() {
        Ok(context) => context,
        Err(err) => panic!("Failed to get GL context: {}", err)
      };
      self.window.gl_make_current(&gl_context);
    }
  }
  let context = unsafe {
      glium::backend::Context::new(Backend { window:rcwindow.clone(), size:dsize }, false)
    }.unwrap();
  
  loop {
    for event in event_pump.poll_iter() {
      use sdl2::event::Event;
      
      let mut target = glium::Frame::new(context.clone(), context.get_framebuffer_dimensions());
      target.clear_color(0.0, 1.0, 0.0, 1.0);
      target.finish();

      match event {
        Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
          return;
        },
        _ => {}
      }
    }
  }
}