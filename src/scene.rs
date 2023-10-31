use std::marker::Sync;
use std::f32::consts::PI;

use windows::Win32::Graphics::Gdi::HDC;

use crate::{Triangle, mesh_stuff::Mesh, vec3_stuff::{Vec3, vec3_mult, vec3_add, vec3_sub}, matrix_stuff::{Mat4x4, matrix_make_rotation_y, multiply_matrix_vector, matrix_point_at, matrix_quick_inverse, matrix_make_rotation_x}, draw_functions::{draw_filled_triangle, draw_triangle}};

#[derive(Debug,Clone)]
pub struct Vec2{
  pub x:f32,
  pub y:f32
}

#[derive(Debug,Clone)]
pub struct Scene{
  pub tris: Vec<Triangle>,
  pub objs: Vec<Mesh>,
  pub camera: Vec3,
  pub mat_camera: Mat4x4,
  pub look_direction: Vec3,
  pub up: Vec3,
  pub prev_mouse: Vec2
}

impl Scene {
  pub fn new() -> Scene {
    Scene{
      tris: vec![],
      objs: vec![],
      camera: Vec3 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
      mat_camera: Mat4x4 { m: [[0.0;4];4] },
      look_direction: Vec3{x:0.0, y:0.0, z:1.0, w:1.0},
      up: Vec3{x:0.0, y:1.0, z:0.0, w:1.0},
      prev_mouse:Vec2 { x: 0.0, y: 0.0 }
    }
  }

  pub fn render(&self, hdc:HDC){
    for tri in &self.tris {
      draw_filled_triangle(hdc, &tri);
      draw_triangle(hdc, &tri);
    }
  }

  pub fn get_scene(&self) -> &Scene {
    self
  }

  pub fn rotate_camera(&self, rotate_amount:Vec3){

  }

  pub fn get_tris(&self) -> Vec<Triangle> {
    self.tris.clone()
  }

  pub fn reset_tris(& mut self){
    self.tris = vec![];
  }

  pub fn add_tri(& mut self, tri:Triangle){
    self.tris.push(tri);
  }

  pub fn get_objs(&self) -> Vec<Mesh> {
    self.objs.clone()
  }

  pub fn get_camera(&self) -> Vec3 {
    self.camera
  }

  pub fn get_mat_camera(&self) -> Mat4x4 {
    self.mat_camera.clone()
  }

  pub fn set_mat_camera(& mut self){
    let target = vec3_add(&self.camera, &self.look_direction);
    self.mat_camera = matrix_quick_inverse(&matrix_point_at(&self.camera, &target, &self.up));
  }

  pub fn get_look_direction(&self) -> Vec3 {
    self.look_direction
  }

  pub fn get_up(&self) -> Vec3 {
    self.up
  }

  pub fn add_obj(& mut self, obj:Mesh){
    self.objs.push(obj);
  }

  pub fn turn_right(& mut self, amount:f32){
    let mat_camera_rotate = matrix_make_rotation_y(amount); 
    self.look_direction = multiply_matrix_vector(&self.look_direction, &mat_camera_rotate);
    self.set_mat_camera();
  }

  pub fn turn_up(& mut self, amount:f32){
    let mat_camera_rotate = matrix_make_rotation_x(amount); 
    self.look_direction = multiply_matrix_vector(&self.look_direction, &mat_camera_rotate);
    self.set_mat_camera();
  }

  pub fn forward(& mut self, amount:f32){
    self.camera = vec3_add(&self.camera, &vec3_mult(&self.look_direction, amount));
    self.set_mat_camera();
  }

  pub fn right(&mut self, amount:f32){
    let mat_direction_rotate = matrix_make_rotation_y(-4.0 / PI);
    self.camera = vec3_add(&self.camera, &vec3_mult(&multiply_matrix_vector(&self.look_direction, &mat_direction_rotate), amount));
    self.set_mat_camera();
  }

  pub fn mouse_move(&mut self, pos:Vec2){
    let delta_x = self.prev_mouse.x - pos.x;
    let delta_y = self.prev_mouse.y - pos.y;
    self.turn_right(delta_x * 0.01);
    self.turn_up(delta_y * 0.01);
    self.prev_mouse = pos;
  }
}