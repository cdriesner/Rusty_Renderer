
#[derive(Debug,Clone,Copy)]
pub struct Vec3 {
  pub x:f32,
  pub y:f32,
  pub z:f32,
  pub w:f32
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