
#[derive(Debug,Clone,Copy)]
pub struct Vec3 {
  pub x:f32,
  pub y:f32,
  pub z:f32,
  pub w:f32
}

impl Vec3 {
  pub fn default() -> Vec3{
    Vec3{
      x:0.0,
      y:0.0,
      z:0.0,
      w:1.0,
    }
  }
}

pub fn vec3_dot_product(v1:&Vec3,v2:&Vec3) -> f32{
  v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}

pub fn vec3_length(v:&Vec3) -> f32 {
  vec3_dot_product(v, v).sqrt()
}

pub fn vec3_normalize(v:&Vec3) -> Vec3 {
  let l = vec3_length(v);
  Vec3 { x: v.x/l, y: v.y/l, z: v.z/l, w:v.w}
}

pub fn vec3_cross_product(v1:&Vec3,v2:&Vec3) -> Vec3 {
  Vec3 { 
    x: v1.y * v2.z - v1.z * v2.y,
    y: v1.z * v2.x - v1.x * v2.z,
    z: v1.x * v2.y - v1.y * v2.x,
    w: 1.0
  }
}

pub fn vec3_add(v1:&Vec3, v2:&Vec3) -> Vec3 {
  Vec3 { 
    x: v1.x + v2.x,
    y: v1.y + v2.y,
    z: v1.z + v2.z, 
    w: 1.0
  }
}

pub fn vec3_sub(v1:&Vec3, v2:&Vec3) -> Vec3 {
  Vec3 { 
    x: v1.x - v2.x,
    y: v1.y - v2.y,
    z: v1.z - v2.z, 
    w: 1.0
  }
}

pub fn vec3_mult(v:&Vec3,k:f32) -> Vec3 {
  Vec3{
    x: v.x * k,
    y: v.y * k,
    z: v.z * k,
    w: v.w
  }
}

pub fn vec3_div(v:&Vec3,k:f32) -> Vec3 {
  Vec3{
    x: v.x / k,
    y: v.y / k,
    z: v.z / k,
    w: v.w
  }
}

pub fn vec3_intersect_plane(plane_p:&Vec3, plane_n: &Vec3, line_start: &Vec3, line_end:&Vec3) -> Vec3{
  let plane_n = vec3_normalize(plane_n);
  let plane_d = -vec3_dot_product(&plane_n, &plane_p);
  let ad = vec3_dot_product(line_start, &plane_n);
  let bd = vec3_dot_product(line_end, &plane_n);
  let t = (-plane_d - ad) / (bd - ad);
  let line_start_to_end = vec3_sub(line_end, line_start);
  let line_to_intersect = vec3_mult(&line_start_to_end, t);
  vec3_add(line_start, &line_to_intersect)
}
