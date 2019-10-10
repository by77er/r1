use super::vec::Vec3;
use super::matrix;
use super::super::camera;

extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Represents a single triangle
pub struct Triangle {
  pub v1: Vec3,
  pub v2: Vec3,
  pub v3: Vec3,
}

impl Triangle {
  /// Creates a new `Triangle` instance
  pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
    Self { v1, v2, v3 }
  }
  /// Draws the triangle to a canvas
  pub fn draw(&self, canvas: &mut Canvas<Window>, origin: Vec3, rotation: Vec3, cam: &camera::Camera) {
    // Transform everything and draw lines
    let tv1 = calc_pt(cam.transform(matrix::rotate(self.v1 + origin, origin, rotation)));
    let tv2 = calc_pt(cam.transform(matrix::rotate(self.v2 + origin, origin, rotation)));
    let tv3 = calc_pt(cam.transform(matrix::rotate(self.v3 + origin, origin, rotation)));
    canvas.draw_line(tv1, tv2).unwrap();
    canvas.draw_line(tv2, tv3).unwrap();
    canvas.draw_line(tv3, tv1).unwrap();
  }
}

fn calc_pt(v: Vec3) -> (i32, i32) {
  (((v.x + 1.0) * 500.0) as i32, ((v.y + 1.0) * 500.0) as i32)
}

/// Represents a grouped set of triangles
pub struct Mesh {
  pub position: Vec3,
  pub rotation: Vec3,
  pub triangles: Vec<Triangle>,
}

impl Mesh {
  /// Creates a new `Mesh` instance
  pub fn new(position: Vec3, triangles: Vec<Triangle>) -> Self {
    Self {
      position,
      rotation: Vec3::new(0.0, 0.0, 0.0),
      triangles
    }
  }
  /// Draw the mesh on the canvas - wireframe
  pub fn draw(&self, canvas: &mut Canvas<Window>, cam: &camera::Camera) {
    // for each triangle, do thing
    for i in self.triangles.iter() {
      i.draw(canvas, self.position, self.rotation, cam);
    }
  }
}

pub fn gen_cube(side_length: f64) -> Vec<Triangle> {
  let mut tris: Vec<Triangle> = Vec::new();
  let back_top_left = Vec3::new(-side_length, side_length, -side_length);
  let back_bottom_left = Vec3::new(-side_length, -side_length, -side_length);
  let back_top_right = Vec3::new(side_length, side_length, -side_length);
  let back_bottom_right = Vec3::new(side_length, -side_length, -side_length);
  let front_top_left = Vec3::new(-side_length, side_length, side_length);
  let front_bottom_left = Vec3::new(-side_length, -side_length, side_length);
  let front_top_right = Vec3::new(side_length, side_length, side_length);
  let front_bottom_right = Vec3::new(side_length, -side_length, side_length);
  
  // back
  tris.push(Triangle::new(back_top_left, back_bottom_right, back_top_right));
  tris.push(Triangle::new(back_top_left, back_bottom_right, back_bottom_left));
  // front
  tris.push(Triangle::new(front_top_left, front_bottom_right, front_top_right));
  tris.push(Triangle::new(front_top_left, front_bottom_right, front_bottom_left));
  // top
  tris.push(Triangle::new(front_top_left, back_top_right, front_top_right));
  tris.push(Triangle::new(front_top_left, back_top_right, back_top_left));
  // bottom
  tris.push(Triangle::new(front_bottom_left, back_bottom_right, front_bottom_right));
  tris.push(Triangle::new(front_bottom_left, back_bottom_right, back_bottom_left));
  // left
  tris.push(Triangle::new(front_bottom_left, back_top_left, front_top_left));
  tris.push(Triangle::new(front_bottom_left, back_top_left, back_bottom_left));
  // right
  tris.push(Triangle::new(front_bottom_right, back_top_right, front_top_right));
  tris.push(Triangle::new(front_bottom_right, back_top_right, back_bottom_right));
  // ret
  tris
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn creation() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let v3 = Vec3::new(0.0, 0.0, 1.0);
    let t = Triangle::new(v1, v2, v3);
    assert_eq!(t.v2.y, 1.0);
  }
}