use super::vec::Vec3;

/// Represents a 3x3 matrix
pub struct Matrix3x3 {
  pub column1: Vec3,
  pub column2: Vec3,
  pub column3: Vec3,
}

impl Matrix3x3 {
  /// Creates a new instance of a 3x3 matrix from three `Vec3`s
  pub fn new(column1: Vec3, column2: Vec3, column3: Vec3) -> Self {
    Self {
      column1,
      column2,
      column3,
    }
  }
  /// Creates a new instance of a 3x3 matrix from 9 values
  pub fn direct_new(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    g: f64,
    h: f64,
    i: f64,
  ) -> Self {
    Self {
      column1: Vec3::new(a, d, g),
      column2: Vec3::new(b, e, h),
      column3: Vec3::new(c, f, i),
    }
  }
  /// Computes the determinant of the matrix
  pub fn determinant(&self) -> f64 {
    self.column1.x * (self.column2.y * self.column3.z - self.column2.z * self.column3.y)
      - self.column2.x * (self.column1.y * self.column3.z - self.column1.z * self.column3.y)
      + self.column3.x * (self.column1.y * self.column2.z - self.column1.z * self.column2.y)
  }
  /// Transforms the given `Vec3` and returns the result of the transformation
  pub fn transform(&self, vector: &Vec3) -> Vec3 {
    Vec3::new(
      self.column1.x * vector.x + self.column2.x * vector.y + self.column3.x * vector.z,
      self.column1.y * vector.x + self.column2.y * vector.y + self.column3.y * vector.z,
      self.column1.z * vector.x + self.column2.z * vector.y + self.column3.z * vector.z,
    )
  }
}

pub fn rotate(point: Vec3, origin: Vec3, rotation: Vec3) -> Vec3 {
  let xr = Matrix3x3::direct_new(
    1.0, 0.0, 0.0,
    0.0, rotation.x.cos(), rotation.x.sin(),
    0.0, -rotation.x.sin(), rotation.x.cos()
  );
  let yr = Matrix3x3::direct_new(
    rotation.y.cos(), 0.0, -rotation.y.sin(),
    0.0, 1.0, 0.0,
    rotation.y.sin(), 0.0, rotation.y.cos()
  );
  let zr = Matrix3x3::direct_new(
    rotation.z.cos(), rotation.z.sin(), 0.0,
    -rotation.z.sin(), rotation.z.cos(), 0.0,
    0.0, 0.0, 1.0
  );
  xr.transform(&yr.transform(&zr.transform(&(point - origin)))) + origin
}

/// Represents a quaternion
pub struct Quaternion {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}

impl Quaternion {
  /// Creates a new quaternion
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
    Self { x, y, z, w }
  }
  /// Returns the magnitude of this quaternion
  pub fn magnitude(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
  }
  /// Returns a normalized version of this quaternion
  pub fn normalize(&self) -> Self {
    let mag = self.magnitude();
    Self::new(self.x / mag, self.y / mag, self.z / mag, self.w / mag)
  }
  /// Multiplies a quaternion by another quaternion
  pub fn mul(&self, other: Self) -> Self {
    Self::new(
      self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
      self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
      self.w * other.y - self.x * other.y + self.y * other.w + self.z * other.x,
      self.w * other.z + self.x * other.z - self.y * other.x + self.z * other.w,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn determinant() {
    let c1 = Vec3::new(0.0, 5.0, 8.0);
    let c2 = Vec3::new(3.0, 2.0, 10.0);
    let c3 = Vec3::new(5.0, 1.0, 6.0);
    let m = Matrix3x3::new(c1, c2, c3);
    assert_eq!(m.determinant(), 104.0);
  }
  #[test]
  fn transform() {
    let c1 = Vec3::new(0.0, 5.0, 8.0);
    let c2 = Vec3::new(3.0, 2.0, 10.0);
    let c3 = Vec3::new(5.0, 1.0, 6.0);
    let m = Matrix3x3::new(c1, c2, c3);
    let v = Vec3::new(4.0, 3.0, 12.0);
    let transformed = m.transform(&v);
    assert_eq!(transformed, Vec3::new(69.0, 38.0, 134.0));
  }
}