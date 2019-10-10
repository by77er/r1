#![allow(dead_code)]

use std::{cmp, fmt, ops};

/// Represents a vector of three elements
#[derive(Copy, Clone)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

// Vec3-specific operations

impl Vec3 {
  /// Creates a new `Vec3` instance
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z }
  }
  /// Computes the dot product of two `Vec3`s
  pub fn dot(&self, other: &Vec3) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
  /// Computes the cross product of two `Vec3`s
  pub fn cross(&self, other: &Vec3) -> Vec3 {
    Vec3::new(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x,
    )
  }
  /// Computes the magnitude of a `Vec3`
  pub fn magnitude(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
  }
  /// Returns a normalized version of a `Vec3`
  pub fn to_unit(&self) -> Vec3 {
    let magnitude = self.magnitude();
    Vec3::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
  }
  /// Returns this vector multiplied by a scalar
  fn mul(&self, scalar: f64) -> Vec3 {
    Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
  }
  /// Multiplies this vector by a scalar
  fn mul_eq(&mut self, scalar: f64) {
    self.x *= scalar;
    self.y *= scalar;
    self.z *= scalar;
  }
  /// Returns this vector divided by a scalar
  fn div(&self, scalar: f64) -> Vec3 {
    Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar)
  }
  /// Divides this vector by a scalar
  fn div_eq(&mut self, scalar: f64) {
    self.x /= scalar;
    self.y /= scalar;
    self.z /= scalar;
  }
}

impl cmp::PartialEq for Vec3 {
  fn eq(&self, other: &Vec3) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
  }
}
impl cmp::Eq for Vec3 {}

impl fmt::Debug for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Vec3: [ x: {}, y: {}, z: {} ]", self.x, self.y, self.z)
  }
}
impl fmt::Display for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[ {}, {}, {} ]", self.x, self.y, self.z)
  }
}

// Operators for Vec3

impl ops::Add for Vec3 {
  type Output = Vec3;
  fn add(self, other: Vec3) -> Vec3 {
    Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
  }
}
impl ops::AddAssign for Vec3 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}
impl ops::Sub for Vec3 {
  type Output = Vec3;
  fn sub(self, other: Vec3) -> Vec3 {
    Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
  }
}
impl ops::SubAssign for Vec3 {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn equality() {
    assert_eq!(Vec3::new(2.0, 4.0, 20.0), Vec3::new(2.0, 4.0, 20.0));
  }
  #[test]
  fn dot() {
    let v1 = Vec3::new(2.0, 4.0, 20.0);
    let v2 = Vec3::new(5.0, 8.0, 2.0);
    assert_eq!(v1.dot(&v2), 82.0);
  }
  #[test]
  fn cross() {
    let v1 = Vec3::new(2.0, 4.0, 20.0);
    let v2 = Vec3::new(5.0, 8.0, 2.0);
    assert_eq!(v1.cross(&v2), Vec3::new(-152.0, 96.0, -4.0));
  }
}