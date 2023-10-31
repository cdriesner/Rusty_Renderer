

mod window;
use std::{ptr::null_mut, rc::Rc, f32::consts::PI, thread::{self, sleep}, time::{Duration, Instant}, i32,fs::{self, File}, io::{BufReader, BufRead}, path::Path};

use window::*;
use windows::{Win32::{UI::WindowsAndMessaging::{GetMessageW, TranslateMessage, DispatchMessageW, MSG, PeekMessageW, PM_REMOVE, PM_NOREMOVE, WM_QUIT}, Graphics::Gdi::{HDC, CreatePen, PS_SOLID, BI_RGB, HPEN, MoveToEx, LineTo, SelectObject, HGDIOBJ, LOGBRUSH, BS_SOLID, ExtCreatePen, PS_COSMETIC, DeleteObject, PS_GEOMETRIC}, Foundation::COLORREF}, Graphics};

mod matrix_stuff;
use matrix_stuff::*;

mod draw_functions;
use draw_functions::*;

mod vec3_stuff;
use vec3_stuff::*;

mod mesh_stuff;
use mesh_stuff::*;

mod scene;
use scene::*;

mod game;
use game::*;

pub static mut GAME:Game = Game{
  screen_height: 800, 
  screen_width: 1000, 
  f_near: 0.1, 
  f_far: 1000.0, 
  f_fov: 90.0, 
  f_aspect_ratio: 800.0 / 1000.0, 
  scene:Scene{
    tris: vec![],
    objs: vec![],
    camera: Vec3 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
    mat_camera: Mat4x4 { m: [[0.0;4];4] },
    look_direction: Vec3{x:0.0, y:0.0, z:1.0, w:1.0},
    up: Vec3{x:0.0, y:1.0, z:0.0, w:1.0},
    prev_mouse: Vec2 { x: 0.0, y: 0.0 }
  }
};
fn main(){

  unsafe { GAME.scene.turn_right(1.0) };

  let cube = Mesh{
    tris: vec![

      // SOUTH
      Triangle{ p:[Vec3{ x:-5.0, y:-5.0, z:-5.0, w:1.0},    Vec3{ x:-5.0, y:5.0, z:-5.0, w:1.0},    Vec3{ x:5.0, y:5.0, z:-5.0, w:1.0}], col:0x1111ff },
      Triangle{ p:[Vec3{ x:-5.0, y:-5.0, z:-5.0, w:1.0},    Vec3{ x:5.0, y:5.0, z:-5.0, w:1.0},    Vec3{ x:5.0, y:-5.0, z:-5.0, w:1.0}], col:0x1111ff },

      // EAST                                                      
      Triangle{ p:[Vec3{ x:5.0, y:-5.0, z:-5.0, w:1.0},    Vec3{ x:5.0, y:5.0, z:-5.0, w:1.0},    Vec3{ x:5.0, y:5.0, z:5.0, w:1.0}], col:0x11A5ff },
      Triangle{ p:[Vec3{ x:5.0, y:-5.0, z:-5.0, w:1.0},    Vec3{ x:5.0, y:5.0, z:5.0, w:1.0},    Vec3{ x:5.0, y:-5.0, z:5.0, w:1.0}], col:0x11A5ff },

      // NORTH                                                     
      Triangle{ p:[Vec3{ x:5.0, y:-5.0, z:5.0, w:1.0},    Vec3{ x:5.0, y:5.0, z:5.0, w:1.0},    Vec3{ x:-5.0, y:5.0, z:5.0, w:1.0}], col:0x11EAff },
      Triangle{ p:[Vec3{ x:5.0, y:-5.0, z:5.0, w:1.0},    Vec3{ x:-5.0, y:5.0, z:5.0, w:1.0},    Vec3{ x:-5.0, y:-5.0, z:5.0, w:1.0}], col:0x11EAff },

      // WEST                                                      
      Triangle{ p:[Vec3{ x:-5.0, y:-5.0, z:5.0, w:1.0},    Vec3{ x:-5.0, y:5.0, z:5.0, w:1.0},    Vec3{ x:-5.0, y:5.0, z:-5.0, w:1.0}], col:0x11ff11 },
      Triangle{ p:[Vec3{ x:-5.0, y:-5.0, z:5.0, w:1.0},    Vec3{ x:-5.0, y:5.0, z:-5.0, w:1.0},    Vec3{ x:-5.0, y:-5.0, z:-5.0, w:1.0}], col:0x11ff11 },

      // TOP                                                       
      Triangle{ p:[Vec3{ x:-5.0, y:5.0, z:-5.0, w:1.0},    Vec3{ x:-5.0, y:5.0, z:5.0, w:1.0},    Vec3{ x:5.0, y:5.0, z:5.0, w:1.0}], col:0xff1111 },
      Triangle{ p:[Vec3{ x:-5.0, y:5.0, z:-5.0, w:1.0},    Vec3{ x:5.0, y:5.0, z:5.0, w:1.0},    Vec3{ x:5.0, y:5.0, z:-5.0, w:1.0}], col:0xff1111 },

      // BOTTOM
      Triangle{ p:[Vec3{ x:5.0, y:-5.0, z:5.0, w:1.0},    Vec3{ x:-5.0, y:-5.0, z:5.0, w:1.0},    Vec3{ x:-5.0, y:-5.0, z:-5.0, w:1.0}], col:0x811181 },
      Triangle{ p:[Vec3{ x:5.0, y:-5.0, z:5.0, w:1.0},    Vec3{ x:-5.0, y:-5.0, z:-5.0, w:1.0},    Vec3{ x:5.0, y:-5.0, z:-5.0, w:1.0}], col:0x811181 },
    ]
  };
  
  let mut done = false;
  
  let screen = Window::new("Hello World",unsafe { &GAME });
  let screen = screen.unwrap();
  let start = Instant::now();

  unsafe { GAME.scene.add_obj(Mesh::mesh_from_obj_file("obj/untitled.obj")) };
  //unsafe{ GAME.scene.add_obj(cube)};

  let mut message = MSG::default();
  unsafe {
    while !done {  
      if PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).into() {
        TranslateMessage(&message);
        DispatchMessageW(&message);
        if message.message == WM_QUIT {
				  done = true;
			  }
      }
      GAME.main_loop(start,&screen);
    }
  }
}

//#[derive(Debug)]





#[derive(Debug,Clone,Copy)]
pub struct Triangle {
  p:[Vec3; 3],
  col:i32
}

impl Triangle {
  pub fn default() -> Triangle{
    Triangle{
      p:[
        Vec3::default(),
        Vec3::default(),
        Vec3::default()
      ],
      col:0
    }
  }
}

pub fn lighten_darken_color(col:i32, amt:f32) -> i32{
  let amt:i32 = unsafe {
    (amt * 100.0 - 50.0).to_int_unchecked::<i32>()
  };

  let mut r = (col >> 16) + amt;
  if r > 255 { r = 255}
  else if  r < 0 {r = 0};
  let mut b = ((col >> 8) & 0x00FF) + amt;
  if b > 255 { b = 255}
  else if  b < 0 {b = 0};
  
  let mut g = (col & 0x0000FF) + amt;
  if g > 255 {g = 255}
  else if g < 0 {g = 0}
  return g | (b << 8) | (r << 16);
}







