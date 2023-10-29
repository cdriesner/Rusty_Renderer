

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


static mut SCENE:Scene = Scene{
  tris: vec![],
  objs: vec![],
};





const SCREEN_HEIGHT:u16 = 400;
const SCREEN_WIDTH:u16 = 400;
const F_NEAR:f32 = 0.1;
const F_FAR:f32 = 1000.0;
const F_FOV:f32 = 90.0;
const F_ASPECT_RATIO: f32 = SCREEN_HEIGHT as f32 / SCREEN_WIDTH as f32;


fn main(){

  // let cube = Mesh{
  //   tris: vec![

  //     // SOUTH
  //     Triangle{ p:[Vec3{ x:-0.5, y:-0.5, z:-0.5},    Vec3{ x:-0.5, y:0.5, z:-0.5},    Vec3{ x:0.5, y:0.5, z:-0.5}], col:0x1111ff },
  //     Triangle{ p:[Vec3{ x:-0.5, y:-0.5, z:-0.5},    Vec3{ x:0.5, y:0.5, z:-0.5},    Vec3{ x:0.5, y:-0.5, z:-0.5}], col:0x1111ff },

  //     // EAST                                                      
  //     Triangle{ p:[Vec3{ x:0.5, y:-0.5, z:-0.5},    Vec3{ x:0.5, y:0.5, z:-0.5},    Vec3{ x:0.5, y:0.5, z:0.5}], col:0x11A5ff },
  //     Triangle{ p:[Vec3{ x:0.5, y:-0.5, z:-0.5},    Vec3{ x:0.5, y:0.5, z:0.5},    Vec3{ x:0.5, y:-0.5, z:0.5}], col:0x11A5ff },

  //     // NORTH                                                     
  //     Triangle{ p:[Vec3{ x:0.5, y:-0.5, z:0.5},    Vec3{ x:0.5, y:0.5, z:0.5},    Vec3{ x:-0.5, y:0.5, z:0.5}], col:0x11EAff },
  //     Triangle{ p:[Vec3{ x:0.5, y:-0.5, z:0.5},    Vec3{ x:-0.5, y:0.5, z:0.5},    Vec3{ x:-0.5, y:-0.5, z:0.5}], col:0x11EAff },

  //     // WEST                                                      
  //     Triangle{ p:[Vec3{ x:-0.5, y:-0.5, z:0.5},    Vec3{ x:-0.5, y:0.5, z:0.5},    Vec3{ x:-0.5, y:0.5, z:-0.5}], col:0x11ff11 },
  //     Triangle{ p:[Vec3{ x:-0.5, y:-0.5, z:0.5},    Vec3{ x:-0.5, y:0.5, z:-0.5},    Vec3{ x:-0.5, y:-0.5, z:-0.5}], col:0x11ff11 },

  //     // TOP                                                       
  //     Triangle{ p:[Vec3{ x:-0.5, y:0.5, z:-0.5},    Vec3{ x:-0.5, y:0.5, z:0.5},    Vec3{ x:0.5, y:0.5, z:0.5}], col:0xff1111 },
  //     Triangle{ p:[Vec3{ x:-0.5, y:0.5, z:-0.5},    Vec3{ x:0.5, y:0.5, z:0.5},    Vec3{ x:0.5, y:0.5, z:-0.5}], col:0xff1111 },

  //     // BOTTOM
  //     Triangle{ p:[Vec3{ x:0.5, y:-0.5, z:0.5},    Vec3{ x:-0.5, y:-0.5, z:0.5},    Vec3{ x:-0.5, y:-0.5, z:-0.5}], col:0x811181 },
  //     Triangle{ p:[Vec3{ x:0.5, y:-0.5, z:0.5},    Vec3{ x:-0.5, y:-0.5, z:-0.5},    Vec3{ x:0.5, y:-0.5, z:-0.5}], col:0x811181 },
  //   ]
  // };
  
  let mut done = false;
  
  let screen = Window::new("Hello World",SCREEN_WIDTH.into(),SCREEN_HEIGHT.into());
  let screen = screen.unwrap();
  let start = Instant::now();

  unsafe { 
    SCENE.objs.push(Mesh::mesh_from_obj_file("obj/untitled.obj"));
    //SCENE.objs.push(cube);
  };
  
  pub fn main_loop(start: Instant,screen: &Box<Window>, scene:&Scene){
    let v_camera:Vec3 = Vec3{
      x:0.0,
      y:0.0,
      z:0.0,
      w:1.0
    };
    unsafe{SCENE.tris = vec![]};
    //let rotate_amount = start.elapsed().as_secs_f32() * 2.0;
    let rotate_amount = 0.0;
    let f_fov_radius:f32 = 1.0 / f32::tan((F_FOV / 2.0) * (PI / 180.0));

    let mat_proj = Mat4x4{
      m: [
        [f_fov_radius * F_ASPECT_RATIO, 0.0, 0.0, 0.0],
        [0.0, f_fov_radius, 0.0, 0.0],
        [0.0, 0.0, F_FAR / (F_FAR - F_NEAR), 1.0],
        [0.0, 0.0, (-F_FAR * F_NEAR) / (F_FAR - F_NEAR), 0.0]
      ]
    };

    let v_up:Vec3 = Vec3{x:0.0, y:0.0, z:0.0, w:1.0};
    let v_look_direction:Vec3 = Vec3{x:0.0, y:1.0, z:0.0, w:1.0};
    let v_target:Vec3 = vec3_add(&v_up, &v_look_direction);
    let mat_camera:Mat4x4 = matrix_point_at(&v_camera, &v_target, &v_up);
    let mat_view:Mat4x4 = matrix_quick_inverse(&mat_camera);

    for obj in &scene.objs{
      for tri in &obj.tris{

        let tri_z_rotate: Triangle = Triangle{
          p:[rotate_z(rotate_amount,&tri.p[0]), 
          rotate_z(rotate_amount,&tri.p[1]), 
          rotate_z(rotate_amount,&tri.p[2])],
          col: tri.col
        };
      
        let tri_x_rotate: Triangle = Triangle{
          p:[rotate_x(rotate_amount * 1.3, &tri_z_rotate.p[0]), 
          rotate_x(rotate_amount * 1.3, &tri_z_rotate.p[1]), 
          rotate_x(rotate_amount * 1.3, &tri_z_rotate.p[2])],
          col: tri_z_rotate.col
        };
      
        let mut tri_translated  = tri_x_rotate;
      
        tri_translated.p[0].z += 10.0;
        tri_translated.p[1].z += 10.0;
        tri_translated.p[2].z += 10.0;
  
        let line1 = Vec3{
          x: tri_translated.p[1].x - tri_translated.p[0].x,
          y: tri_translated.p[1].y - tri_translated.p[0].y,
          z: tri_translated.p[1].z - tri_translated.p[0].z,
          w: 1.0
        };
  
        let line2 = Vec3{
          x: tri_translated.p[2].x - tri_translated.p[0].x,
          y: tri_translated.p[2].y - tri_translated.p[0].y,
          z: tri_translated.p[2].z - tri_translated.p[0].z,
          w: 1.0
        };
  
        let mut normal = vec3_cross_product(&line1, &line2);
  
        normal = vec3_normalize(&normal); 

        let v_camera_ray:Vec3 = vec3_sub(&tri_translated.p[0], &v_camera);

        if vec3_dot_product(&normal, &v_camera_ray) < 0.0 {

          let light_direction = Vec3{x:-1.0,y:-1.0, z:0.0, w:1.0};
          let dp = vec3_dot_product(&light_direction, &normal);
          tri_translated.col = lighten_darken_color(tri_translated.col, dp);
  
  
          let mut tri_projected: Triangle = Triangle{
            p:[
              multiply_matrix_vector(&tri_translated.p[0], &mat_proj), 
              multiply_matrix_vector(&tri_translated.p[1], &mat_proj), 
              multiply_matrix_vector(&tri_translated.p[2], &mat_proj)
            ],
            col:tri_translated.col
          };

          tri_projected.p[0] = vec3_div(&tri_projected.p[0], tri_projected.p[0].w);
          tri_projected.p[1] = vec3_div(&tri_projected.p[1], tri_projected.p[1].w);
          tri_projected.p[2] = vec3_div(&tri_projected.p[2], tri_projected.p[2].w);
  
          tri_projected.p[0].x += 1.0; tri_projected.p[0].y += 1.0;
          tri_projected.p[1].x += 1.0; tri_projected.p[1].y += 1.0;
          tri_projected.p[2].x += 1.0; tri_projected.p[2].y += 1.0;
          tri_projected.p[0].x *= 0.5 * SCREEN_WIDTH as f32; tri_projected.p[0].y *= 0.5 * SCREEN_HEIGHT as f32;
          tri_projected.p[1].x *= 0.5 * SCREEN_WIDTH as f32; tri_projected.p[1].y *= 0.5 * SCREEN_HEIGHT as f32;
          tri_projected.p[2].x *= 0.5 * SCREEN_WIDTH as f32; tri_projected.p[2].y *= 0.5 * SCREEN_HEIGHT as f32;
          unsafe{SCENE.tris.push(tri_projected)};
        }
      }
    }
    screen.paint_window();
    sleep(Duration::new(0,50000000));
  }
  
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
      main_loop(start,&screen,&SCENE);
    }
  }
}

//#[derive(Debug)]
pub struct Scene{
  tris: Vec<Triangle>,
  objs: Vec<Mesh>
}

impl Scene {
  pub fn render(&self, hdc:HDC){
    for tri in &self.tris {
      draw_filled_triangle(hdc, &tri);
    }
  }

  pub fn get_scene(&self) -> &Scene {
    self
  }
}




#[derive(Debug,Clone)]
pub struct Triangle {
  p:[Vec3; 3],
  col:i32
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







