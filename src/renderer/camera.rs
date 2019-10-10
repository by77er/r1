use super::types::vec::Vec3;
use super::types::matrix::Matrix3x3;

/// Represents a camera
pub struct Camera {
  pub position: Vec3,
  pub surface_offset: Vec3, // Relative to the camera position
  pub rotation: Vec3,
  x_rotation_matrix: Matrix3x3,
  y_rotation_matrix: Matrix3x3,
  z_rotation_matrix: Matrix3x3,
  projection_matrix: Matrix3x3,
}

impl Camera {
  /// Creates a new `Camera` instance
  pub fn new(position: Vec3, fov: f64, rotation: Vec3) -> Self {
    // Calculate rotation matrices
    let x_rotation_matrix = Matrix3x3::direct_new(
      1.0, 0.0, 0.0,
      0.0, rotation.x.cos(), rotation.x.sin(),
      0.0, -rotation.x.sin(), rotation.x.cos()
    );
    let y_rotation_matrix = Matrix3x3::direct_new(
      rotation.y.cos(), 0.0, -rotation.y.sin(),
      0.0, 1.0, 0.0,
      rotation.y.sin(), 0.0, rotation.y.cos()
    );
    let z_rotation_matrix = Matrix3x3::direct_new(
      rotation.z.cos(), rotation.z.sin(), 0.0,
      -rotation.z.sin(), rotation.z.cos(), 0.0,
      0.0, 0.0, 1.0
    );

    let surface_offset = Vec3::new(0.0, 0.0, 1.0 / (fov / 2.0).tan());

    let projection_matrix = Matrix3x3::direct_new(
      1.0, 0.0, surface_offset.x / surface_offset.z,
      0.0, 1.0, surface_offset.y / surface_offset.z,
      0.0, 0.0, 1.0 / surface_offset.z
    );
    Self {
      position,
      surface_offset,
      rotation,
      x_rotation_matrix,
      y_rotation_matrix,
      z_rotation_matrix,
      projection_matrix
    }
  }

  pub fn recalculate_matrices(&mut self) {
    self.x_rotation_matrix = Matrix3x3::direct_new(
      1.0, 0.0, 0.0,
      0.0, self.rotation.x.cos(), self.rotation.x.sin(),
      0.0, -self.rotation.x.sin(), self.rotation.x.cos()
    );
    self.y_rotation_matrix = Matrix3x3::direct_new(
      self.rotation.y.cos(), 0.0, -self.rotation.y.sin(),
      0.0, 1.0, 0.0,
      self.rotation.y.sin(), 0.0, self.rotation.y.cos()
    );
    self.z_rotation_matrix = Matrix3x3::direct_new(
      self.rotation.z.cos(), self.rotation.z.sin(), 0.0,
      -self.rotation.z.sin(), self.rotation.z.cos(), 0.0,
      0.0, 0.0, 1.0
    );
  }
  /// Transforms a point from 3D space to 2D space
  /// Vec3 input contains x, y, z
  /// Vec3 output contains x, y, distance from camera
  pub fn transform(&self, point: Vec3) -> Vec3 {
    // Offset from camera
    let offset_vector = point - self.position;
    // Rotation transformations
    let rotated_vector = self.x_rotation_matrix.transform(
      &self.y_rotation_matrix.transform(
        &self.z_rotation_matrix.transform(
          &offset_vector
        )
      )
    );

    let projected_vector = self.projection_matrix.transform(&rotated_vector);

    // Transform to screen coordinates
    Vec3::new(
      projected_vector.x / projected_vector.z,
      projected_vector.y / projected_vector.z,
      projected_vector.z
      )

  }
}