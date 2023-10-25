

mod window;
use std::{ptr::null_mut, rc::Rc, f32::consts::PI, thread, time::Duration};

use window::*;
use windows::{Win32::{UI::WindowsAndMessaging::{GetMessageW, TranslateMessage, DispatchMessageW, MSG}, Graphics::Gdi::{HDC, CreatePen, PS_SOLID, BI_RGB, HPEN, MoveToEx, LineTo, SelectObject, HGDIOBJ, LOGBRUSH, BS_SOLID, ExtCreatePen, PS_COSMETIC, DeleteObject, PS_GEOMETRIC}, Foundation::COLORREF}, Graphics};

static mut SCENE:Scene = Scene{
  tris: vec![]
};

pub fn get_scene() -> &'static Scene {
  unsafe{
    &SCENE
  }
}

fn main(){
  let cube = Mesh{
    tris: vec![

	  	// SOUTH
	  	Triangle( Vec3{ x:0.0, y:0.0, z:0.0},    Vec3{ x:0.0, y:1.0, z:0.0},    Vec3{ x:1.0, y:1.0, z:0.0} ),
	  	Triangle( Vec3{ x:0.0, y:0.0, z:0.0},    Vec3{ x:1.0, y:1.0, z:0.0},    Vec3{ x:1.0, y:0.0, z:0.0} ),

	  	// EAST                                                      
	  	Triangle( Vec3{ x:1.0, y:0.0, z:0.0},    Vec3{ x:1.0, y:1.0, z:0.0},    Vec3{ x:1.0, y:1.0, z:1.0} ),
	  	Triangle( Vec3{ x:1.0, y:0.0, z:0.0},    Vec3{ x:1.0, y:1.0, z:1.0},    Vec3{ x:1.0, y:0.0, z:1.0} ),

	  	// NORTH                                                     
	  	Triangle( Vec3{ x:1.0, y:0.0, z:1.0},    Vec3{ x:1.0, y:1.0, z:1.0},    Vec3{ x:0.0, y:1.0, z:1.0} ),
	  	Triangle( Vec3{ x:1.0, y:0.0, z:1.0},    Vec3{ x:0.0, y:1.0, z:1.0},    Vec3{ x:0.0, y:0.0, z:1.0} ),

	  	// WEST                                                      
	  	Triangle( Vec3{ x:0.0, y:0.0, z:1.0},    Vec3{ x:0.0, y:1.0, z:1.0},    Vec3{ x:0.0, y:1.0, z:0.0} ),
	  	Triangle( Vec3{ x:0.0, y:0.0, z:1.0},    Vec3{ x:0.0, y:1.0, z:0.0},    Vec3{ x:0.0, y:0.0, z:0.0} ),

	  	// TOP                                                       
	  	Triangle( Vec3{ x:0.0, y:1.0, z:0.0},    Vec3{ x:0.0, y:1.0, z:1.0},    Vec3{ x:1.0, y:1.0, z:1.0} ),
	  	Triangle( Vec3{ x:0.0, y:1.0, z:0.0},    Vec3{ x:1.0, y:1.0, z:1.0},    Vec3{ x:1.0, y:1.0, z:0.0} ),

	  	// BOTTOM
	  	Triangle( Vec3{ x:1.0, y:0.0, z:1.0},    Vec3{ x:0.0, y:0.0, z:1.0},    Vec3{ x:0.0, y:0.0, z:0.0} ),
	  	Triangle( Vec3{ x:1.0, y:0.0, z:1.0},    Vec3{ x:0.0, y:0.0, z:0.0},    Vec3{ x:1.0, y:0.0, z:0.0} ),
    ]
  };
  
  let screen_height:u16 = 400;
  let screen_width:u16 = 400;
  let f_near = 0.1;
  let f_far = 1000.0;
  let f_fov = 90.0;
  let f_aspect_ratio: f32 = screen_height as f32 / screen_width as f32;
  let f_fov_radius = 1.0 / f32::tan((f_fov / 2.0) * (PI / 180.0));

  let screen = Window::new("Hello World",screen_width.into(),screen_height.into());
  
  let handle = thread::spawn(move || {
    let mut rotate_amount = 1.0;
    let s = screen.unwrap();
    loop {
      print!("ran");
      let mat_proj = Mat4x4{
        m: [
          [f_fov_radius * f_aspect_ratio, 0.0, 0.0, 0.0],
          [0.0, f_fov_radius, 0.0, 0.0],
          [0.0, 0.0, f_far / (f_far - f_near), 1.0],
          [0.0, 0.0, (-f_far * f_near) / (f_far - f_near), 0.0]
        ]
      };
    
      let mat_rotate_x = Mat4x4{
        m: [
          [1.0, 0.0, 0.0, 0.0],
          [0.0, f32::cos(rotate_amount * 0.5), f32::sin(rotate_amount * 0.5),0.0],
          [0.0, -f32::sin(rotate_amount * 0.5),f32::cos(rotate_amount * 0.5),0.0],
          [0.0,0.0,0.0,1.0]
        ]
      };
    
      let mat_rotate_z = Mat4x4{
        m: [
          [f32::cos(rotate_amount * 0.5), f32::sin(rotate_amount * 0.5), 0.0, 0.0],
          [-f32::sin(rotate_amount * 0.5), f32::cos(rotate_amount * 0.5), 0.0,0.0],
          [0.0, 0.0,1.0,0.0],
          [0.0,0.0,0.0,1.0]
        ]
      };
      for tri in &cube.tris{
    
        let tri_z_rotate: Triangle = Triangle(
          multiply_matrix_vector(&tri.0, &mat_rotate_z), 
          multiply_matrix_vector(&tri.1, &mat_rotate_z), 
          multiply_matrix_vector(&tri.2, &mat_rotate_z)
        );
    
        let tri_x_rotate: Triangle = Triangle(
          multiply_matrix_vector(&tri_z_rotate.0, &mat_rotate_x), 
          multiply_matrix_vector(&tri_z_rotate.1, &mat_rotate_x), 
          multiply_matrix_vector(&tri_z_rotate.2, &mat_rotate_x)
        );
    
        let mut tri_translated  = tri_x_rotate;
    
        tri_translated.0.z += 3.0;
        tri_translated.1.z += 3.0;
        tri_translated.2.z += 3.0;
        let mut tri_projected: Triangle = Triangle(
          multiply_matrix_vector(&tri_translated.0, &mat_proj), 
          multiply_matrix_vector(&tri_translated.1, &mat_proj), 
          multiply_matrix_vector(&tri_translated.2, &mat_proj)
        );
        tri_projected.0.x += 1.0; tri_projected.0.y += 1.0;
        tri_projected.1.x += 1.0; tri_projected.1.y += 1.0;
        tri_projected.2.x += 1.0; tri_projected.2.y += 1.0;
        tri_projected.0.x *= 0.5 * screen_width as f32; tri_projected.0.y *= 0.5 * screen_height as f32;
        tri_projected.1.x *= 0.5 * screen_width as f32; tri_projected.1.y *= 0.5 * screen_height as f32;
        tri_projected.2.x *= 0.5 * screen_width as f32; tri_projected.2.y *= 0.5 * screen_height as f32;
        unsafe{SCENE.tris.push(tri_projected)};
      }
      rotate_amount += 1.1;
      s.redraw_window();
      s.paint_window();
      thread::sleep(Duration::from_millis(100));
    }
  });
  let mut message = MSG::default();
  unsafe {
    while GetMessageW(&mut message, None, 0, 0).into() {
      TranslateMessage(&message);
      DispatchMessageW(&message);
      thread::sleep(Duration::from_millis(100));
    }
  }
}

pub fn draw_line(hdc:HDC, start_x:f32, start_y:f32, end_x:f32, end_y:f32){
  let rounded_start_x:i32 = unsafe{start_x.to_int_unchecked::<i32>()};
  let rounded_start_y:i32 = unsafe{start_y.to_int_unchecked::<i32>()};
  let rounded_end_x:i32 = unsafe{end_x.to_int_unchecked::<i32>()};
  let rounded_end_y:i32 = unsafe{end_y.to_int_unchecked::<i32>()};
  let pen = unsafe{CreatePen(PS_SOLID, 2, COLORREF(0x000000ff))};
  let h_pen_old: HGDIOBJ = unsafe{SelectObject(hdc, pen)};
  unsafe{MoveToEx(hdc, rounded_start_x, rounded_start_y, None)};
  unsafe{LineTo(hdc, rounded_end_x, rounded_end_y)};
  unsafe { SelectObject(hdc, h_pen_old) };
  unsafe{ DeleteObject(pen) };
}

pub fn draw_triangle(hdc:HDC, tri:&Triangle){
  draw_line(hdc, tri.0.x, tri.0.y, tri.1.x, tri.1.y);
  draw_line(hdc, tri.1.x, tri.1.y, tri.2.x, tri.2.y);
  draw_line(hdc, tri.2.x, tri.2.y, tri.0.x, tri.0.y);
}

#[derive(Debug)]
pub struct Scene{
  tris: Vec<Triangle>
}

impl Scene {
  pub fn render(&self, hdc:HDC){
    for tri in &self.tris {
      draw_triangle(hdc, &tri);
    }
  }
}

#[derive(Debug,Clone)]
pub struct Vec3 {
  x:f32,
  y:f32,
  z:f32
}

#[derive(Debug,Clone)]
pub struct Triangle (Vec3,Vec3,Vec3);

#[derive(Debug,Clone)]
pub struct Mesh {
  tris: Vec<Triangle>
}

pub struct Mat4x4 {m:[[f32; 4]; 4]}

pub fn multiply_matrix_vector(input: &Vec3, matrix: &Mat4x4) -> Vec3 {
  let mut output: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
  //println!("in x:{} y:{} z:{} ",input.x,input.y,input.z);
  output.x = input.x * matrix.m[0][0] + input.y * matrix.m[1][0] + input.z * matrix.m[2][0] + matrix.m[3][0];
  output.y = input.x * matrix.m[0][1] + input.y * matrix.m[1][1] + input.z * matrix.m[2][1] + matrix.m[3][1];
  output.z = input.x * matrix.m[0][2] + input.y * matrix.m[1][2] + input.z * matrix.m[2][2] + matrix.m[3][2];
  let w: f32 = input.x * matrix.m[0][3] + input.y * matrix.m[1][3] + input.z * matrix.m[2][3] + matrix.m[3][3];
  //println!("2nd y:{} w:{}", output.y,w);
  if w != 0.0 {
    output.x /= w;
    output.y /= w;
    output.z /= w;
  }
  //println!("out x:{} y:{} z:{} w:{}",output.x,output.y,output.x,w);
  //println!("");
  output
}




