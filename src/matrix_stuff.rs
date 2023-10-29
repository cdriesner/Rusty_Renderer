use crate::*;

pub struct Mat4x4 {pub m:[[f32; 4]; 4]}

pub fn rotate_x(rotate_amount:f32, point:&Vec3) -> Vec3{
  let mat_rotate_x = Mat4x4{
    m: [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, f32::cos(rotate_amount * 0.5), f32::sin(rotate_amount * 0.5),0.0],
      [0.0, -f32::sin(rotate_amount * 0.5),f32::cos(rotate_amount * 0.5),0.0],
      [0.0,0.0,0.0,1.0]
    ]
  };
  multiply_matrix_vector(&point, &mat_rotate_x)
}

pub fn rotate_z(rotate_amount:f32, point:&Vec3) -> Vec3{
  let mat_rotate_z = Mat4x4{
    m: [
      [f32::cos(rotate_amount * 0.5), f32::sin(rotate_amount * 0.5), 0.0, 0.0],
      [-f32::sin(rotate_amount * 0.5), f32::cos(rotate_amount * 0.5), 0.0,0.0],
      [0.0, 0.0,1.0,0.0],
      [0.0,0.0,0.0,1.0]
    ]
  };
  multiply_matrix_vector(&point, &mat_rotate_z)
}

pub fn multiply_matrix_vector(input: &Vec3, matrix: &Mat4x4) -> Vec3 {
  let mut output: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0, w:1.0 };
  output.x = input.x * matrix.m[0][0] + input.y * matrix.m[1][0] + input.z * matrix.m[2][0] + input.w * matrix.m[3][0];
  output.y = input.x * matrix.m[0][1] + input.y * matrix.m[1][1] + input.z * matrix.m[2][1] + input.w * matrix.m[3][1];
  output.z = input.x * matrix.m[0][2] + input.y * matrix.m[1][2] + input.z * matrix.m[2][2] + input.w * matrix.m[3][2];
  output.w = input.x * matrix.m[0][3] + input.y * matrix.m[1][3] + input.z * matrix.m[2][3] + input.w * matrix.m[3][3];
  output
}

pub fn matrix_point_at(pos:&Vec3,target:&Vec3,up:&Vec3) -> Mat4x4 {
  let new_forward:Vec3 = vec3_sub(target, pos);
  let new_forward:Vec3 = vec3_normalize(&new_forward);

  let a:Vec3 = vec3_mult(&new_forward, vec3_dot_product(up, &new_forward));
  let new_up:Vec3 = vec3_sub(up, &a);
  let new_up:Vec3 = vec3_normalize(&new_up);

  let new_right:Vec3 = vec3_cross_product(&new_up, &new_forward);

  Mat4x4{
    m:[
      [new_right.x, new_right.y, new_right.z, 0.0],
	  	[new_up.x, new_up.y, new_up.z, 0.0],
	  	[new_forward.x, new_forward.y, new_forward.z, 0.0],
	  	[pos.x, pos.y, pos.z, 1.0],
    ]
  }
}

pub fn matrix_quick_inverse(m:&Mat4x4) -> Mat4x4 {
  Mat4x4 { m: [
    [m.m[0][0],m.m[0][0],m.m[0][0],0.0],
    [m.m[0][1],m.m[0][1],m.m[0][1],0.0],
    [m.m[0][2],m.m[0][2],m.m[0][2],0.0],
    [
      -(m.m[3][0] * m.m[0][0] + m.m[3][1] * m.m[0][1] + m.m[3][2] * m.m[0][2]),
      -(m.m[3][0] * m.m[1][0] + m.m[3][1] * m.m[1][1] + m.m[3][2] * m.m[1][2]),
      -(m.m[3][0] * m.m[2][0] + m.m[3][1] * m.m[2][1] + m.m[3][2] * m.m[2][2]),
      1.0
    ]
  ]}
}
