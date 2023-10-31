use std::{time::{Instant, Duration}, f32::consts::PI, thread::sleep};

use windows::Win32::{UI::WindowsAndMessaging::WM_KEYDOWN, Foundation::WPARAM};

use crate::{scene::Scene, window::Window, matrix_stuff::{Mat4x4, rotate_z, rotate_x, multiply_matrix_vector}, Triangle, vec3_stuff::{Vec3, vec3_cross_product, vec3_normalize, vec3_sub, vec3_dot_product, vec3_div, vec3_intersect_plane}, lighten_darken_color};

#[derive(Debug,Clone)]
pub struct Game {
  pub screen_height:u16,
  pub screen_width:u16,
  pub f_near:f32,
  pub f_far:f32,
  pub f_fov:f32,
  pub f_aspect_ratio:f32,
  pub scene:Scene,
}

impl Game {
  pub fn new(h:u16,w:u16,near:f32,far:f32,fov:f32) -> Game {
    Game { 
      screen_height: h, 
      screen_width: w, 
      f_near: near, 
      f_far: far, 
      f_fov: fov, 
      f_aspect_ratio: h as f32 / w as f32, 
      scene:Scene::new()
    }
  }

  pub fn key_down(&mut self, key: WPARAM) {
    match key {
      //w
      WPARAM(0x57) => {
        self.scene.forward(0.1);
      },
      //a
      WPARAM(0x41) => {
        self.scene.right(-0.1);
      }
      //s
      WPARAM(0x53) => {
        self.scene.forward(-0.1);
      }
      //d
      WPARAM(0x44) => {
        self.scene.right(0.1);
      }
      _=>()
    };
  }

  pub fn main_loop(&mut self, start: Instant,screen: &Box<Window>){
    self.scene.reset_tris();
    let rotate_amount = 0.0;
    //unsafe{self.scene.turn(0.1)}
    //self.scene.forward(0.1);
    let f_fov_radius:f32 = 1.0 / f32::tan((self.f_fov / 2.0) * (PI / 180.0));

    let mat_proj = Mat4x4{
      m: [
        [f_fov_radius * self.f_aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f_fov_radius, 0.0, 0.0],
        [0.0, 0.0, self.f_far / (self.f_far - self.f_near), 1.0],
        [0.0, 0.0, (-self.f_far * self.f_near) / (self.f_far - self.f_near), 0.0]
      ]
    };

    let mut tris_to_raster:Vec<Triangle> = vec![];

    for obj in self.scene.get_objs(){
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

        let v_camera_ray:Vec3 = vec3_sub(&tri_translated.p[0], &self.scene.get_camera());

        if vec3_dot_product(&normal, &v_camera_ray) < 0.0 {

          let light_direction = Vec3{x:0.0,y:1.0, z:-1.0, w:1.0};
          let light_direction = vec3_normalize(&light_direction);
          let dp = vec3_dot_product(&light_direction, &normal);
          tri_translated.col = lighten_darken_color(tri_translated.col, dp);
          
          let tri_viewed:Triangle = Triangle { 
            p: [
              multiply_matrix_vector(&tri_translated.p[0],&self.scene.get_mat_camera()),
              multiply_matrix_vector(&tri_translated.p[1],&self.scene.get_mat_camera()),
              multiply_matrix_vector(&tri_translated.p[2],&self.scene.get_mat_camera())
            ], 
            col: tri_translated.col 
          };

          let mut clipped1 = Triangle::default();
          let mut clipped2 = Triangle::default();
          let near_plane = Vec3{
            x:0.0,
            y:0.0,
            z:0.1,
            w:1.0
          };
          let normal_plane = Vec3{
            x:0.0,
            y:0.0,
            z:1.0,
            w:1.0
          };
          let (is_original,n_clipped_triangles) = triangle_clip_against_plane(&near_plane, &normal_plane, &tri_viewed, &mut clipped1, &mut clipped2);
          let mut clipped:[Triangle; 2] = [clipped1,clipped2];
          if is_original {
            clipped[0] = tri_viewed;
          }
          //println!("{}",is_original);

          for i  in 0..n_clipped_triangles {
            //println!("{}",i);
            let mut tri_projected: Triangle = Triangle{
              p:[
                multiply_matrix_vector(&clipped[i as usize].p[0], &mat_proj), 
                multiply_matrix_vector(&clipped[i as usize].p[1], &mat_proj), 
                multiply_matrix_vector(&clipped[i as usize].p[2], &mat_proj)
              ],
              col:clipped[i as usize].col
            };

            // tri_projected.p[0].x *= -1.0;
					  // tri_projected.p[1].x *= -1.0;
					  // tri_projected.p[2].x *= -1.0;
					  // tri_projected.p[0].y *= -1.0;
					  // tri_projected.p[1].y *= -1.0;
					  // tri_projected.p[2].y *= -1.0;
  
            tri_projected.p[0] = vec3_div(&tri_projected.p[0], tri_projected.p[0].w);
            tri_projected.p[1] = vec3_div(&tri_projected.p[1], tri_projected.p[1].w);
            tri_projected.p[2] = vec3_div(&tri_projected.p[2], tri_projected.p[2].w);
    
            tri_projected.p[0].x += 1.0; tri_projected.p[0].y += 1.0;
            tri_projected.p[1].x += 1.0; tri_projected.p[1].y += 1.0;
            tri_projected.p[2].x += 1.0; tri_projected.p[2].y += 1.0;
            tri_projected.p[0].x *= 0.5 * self.screen_width as f32; tri_projected.p[0].y *= 0.5 * self.screen_height as f32;
            tri_projected.p[1].x *= 0.5 * self.screen_width as f32; tri_projected.p[1].y *= 0.5 * self.screen_height as f32;
            tri_projected.p[2].x *= 0.5 * self.screen_width as f32; tri_projected.p[2].y *= 0.5 * self.screen_height as f32;

             

            tris_to_raster.push(tri_projected);
          }
        }
      }
    }

    for tri in tris_to_raster {
      let mut que:Vec<Triangle> = vec![];

      let mut cliped1 = Triangle::default();
      let mut cliped2 = Triangle::default();

      que.push(tri);

      let mut n_new_tris = 1;

      for p in 0..4 {
        let mut n_tris_to_add = 0;
        while n_new_tris > 0 {
          let test:Triangle = que.pop().unwrap();
          n_new_tris -= 1;
          let mut is_original = false;
          match p {
            0 => ( is_original , n_tris_to_add) = triangle_clip_against_plane(&Vec3::default(),&Vec3{x:0.0,y:1.0,z:0.0,w:1.0}, &test, &mut cliped1, &mut cliped2),
            1 => ( is_original , n_tris_to_add) = triangle_clip_against_plane(&Vec3{x:0.0,y:(self.screen_height - 1) as f32,z:0.0,w:1.0},&Vec3{x:0.0,y:-1.0,z:0.0,w:1.0}, &test, &mut cliped1, &mut cliped2),
            2 => ( is_original , n_tris_to_add) = triangle_clip_against_plane(&Vec3{x:0.0,y:0.0,z:0.0,w:1.0},&Vec3{x:1.0,y:0.0,z:0.0,w:1.0}, &test, &mut cliped1, &mut cliped2),
            3 => ( is_original , n_tris_to_add) = triangle_clip_against_plane(&Vec3{x:(self.screen_width - 1) as f32,y:0.0,z:0.0,w:1.0},&Vec3{x:-1.0,y:0.0,z:0.0,w:1.0}, &test, &mut cliped1, &mut cliped2),
            _ => ()
          }
          let mut cliped = [cliped1,cliped2];
          if is_original {
            cliped[0] = test;
          }
          for w in 0..n_tris_to_add {
            que.push(cliped[w as usize]);
          }
        }
        n_new_tris = que.len() as i32;
      }
      for tri in que {
        self.scene.add_tri(tri);
      }
    }
    screen.paint_window();
    sleep(Duration::new(0,50000000));
  }
}


pub fn triangle_clip_against_plane(plane_p:&Vec3, plane_n:&Vec3, in_tri:&Triangle, out_tri1:&mut Triangle, out_tri2:&mut Triangle) -> (bool,i32){
  let plane_n = vec3_normalize(plane_n);
  let dist = |p:&Vec3| -> f32 {
    let p = vec3_normalize(p);
    plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - vec3_dot_product(&plane_n, plane_p)
  };

  let mut inside_points:Vec<&Vec3> = vec![]; let mut n_inside_point_count = 0;
  let mut outside_points:Vec<&Vec3> = vec![]; let mut n_outside_point_count = 0;

  let d0 = dist(&in_tri.p[0]);
  let d1 = dist(&in_tri.p[1]);
  let d2 = dist(&in_tri.p[2]);

  if d0 >= 0.0 { 
    n_inside_point_count+=1;
    inside_points.push(&in_tri.p[0]); 
  } else {
    n_outside_point_count+=1;
    outside_points.push(&in_tri.p[0]); 
  }

	if d1 >= 0.0 {
    n_inside_point_count+=1;
    inside_points.push(&in_tri.p[1]); 
  } else { 
    n_outside_point_count+=1;
    outside_points.push(&in_tri.p[1]); 
  }

	if d2 >= 0.0 { 
    n_inside_point_count+=1;
    inside_points.push(&in_tri.p[2]); 
  }else { 
    n_outside_point_count+=1;
    outside_points.push(&in_tri.p[2]); 
  }

  //println!("in:{} out:{}",n_inside_point_count,n_outside_point_count);

  if  n_inside_point_count == 0 {
		return (false,0); 
	}

  if n_inside_point_count == 3 {
    return (true,1);
  }

  if n_inside_point_count == 1 && n_outside_point_count == 2 {
    out_tri1.col = in_tri.col;

    out_tri1.p[0] = inside_points[0].clone();

    out_tri1.p[1] = vec3_intersect_plane(plane_p, &plane_n, &inside_points[0], &outside_points[0]);
		out_tri1.p[2] = vec3_intersect_plane(plane_p, &plane_n, &inside_points[0], &outside_points[1]);

    out_tri1.col = 0xff00ff;

    return (false,1);
  }

  if n_inside_point_count == 2 && n_outside_point_count == 1 {
    out_tri1.col =  in_tri.col;
    out_tri2.col =  in_tri.col;

    out_tri1.p[0] = inside_points[0].clone();
		out_tri1.p[1] = inside_points[1].clone();
		out_tri1.p[2] = vec3_intersect_plane(plane_p, &plane_n, &inside_points[0], &outside_points[0]);

    out_tri1.col = 0x00ff00;
		
		out_tri2.p[0] = inside_points[1].clone();
		out_tri2.p[1] = out_tri1.p[2].clone();
		out_tri2.p[2] = vec3_intersect_plane(plane_p, &plane_n, &inside_points[1], &outside_points[0]);

    out_tri2.col = 0xff0000;

    return (false,2);
  }

  return (false,0);
}