use std::{path::Path, fs::File, io::{BufReader, BufRead}};

use crate::{Triangle, vec3_stuff::Vec3};

#[derive(Debug,Clone)]
pub struct Mesh {
  pub tris: Vec<Triangle>,
}

impl Mesh{
  pub fn mesh_from_obj_file(name: &str) -> Mesh{
    let mut verts:Vec<Vec3> = vec![];
    let mut tris:Vec<Triangle> = vec![];
    let lines = lines_from_file(Path::new(name));
    for line in lines{
      let parts:Vec<&str> = line.split(" ").collect();
      match parts[0] {
        "v" => {
          verts.push(
            Vec3 { 
              x: parts[1].parse::<f32>().unwrap(),
              y: parts[2].parse::<f32>().unwrap(),
              z: parts[3].parse::<f32>().unwrap(),
              w: 1.0
             }
          )
        }
        "f" => {
          let mut v:Vec<&str> = vec![];
          let mut vt:Vec<&str> = vec![];
          let mut vn:Vec<&str> = vec![];
          let mut i = 1;
          while i <= 3 {
            let nums:Vec<&str> = parts[i].split("/").collect();
            v.push(nums[0]);
            vt.push(nums[1]);
            vn.push(nums[2]);
            i += 1;
          }
          tris.push(
            Triangle { 
              p: 
                [
                  verts[v[0].parse::<usize>().unwrap() - 1],
                  verts[v[1].parse::<usize>().unwrap() - 1],
                  verts[v[2].parse::<usize>().unwrap() - 1]
                ], 
              col: 0x122ff45 
            }
          )
        }
        _=>()
      }
    }
    return Mesh{tris};
  }
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
  let file = File::open(filename).expect("no such file");
  let buf = BufReader::new(file);
  buf.lines()
    .map(|l| l.expect("Could not parse line"))
    .collect()
}