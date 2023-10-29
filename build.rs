use cc;

fn main() {
  print!("ran");
  cc::Build::new()
    .cpp(true)
    .file("c_src/functions.cpp")
    .compile("functions");
}