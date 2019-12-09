//!
//! Geometry helper functionality.
use crate::Vec3;

/// A plane which can be intersected by a ray.
#[derive(Debug, Copy, Clone)]
pub struct Plane {
    /// f32he plane described as x,y,z normal
    normal: Vec3,
    /// dot product of the point and normal, representing the plane position
    bias: f32,
}
impl Plane {
    /// Create a new `Plane`.
    pub fn new(normal: Vec3, bias: f32) -> Self {
        Plane { normal, bias }
    }

    /// Create a new `Plane` from a point normal representation
    pub fn from_point_normal(point: &Vec3, normal: &Vec3) -> Self {
        let normalized = normal.normalized();
        Self {
            normal: Vec3::new(normalized.x, normalized.y, normalized.z),
            bias: point.dot(normalized),
        }
    }

    /// Create a new `Plane` from a point normal representation
    pub fn from_point_vectors(point: &Vec3, v1: &Vec3, v2: &Vec3) -> Self {
        Self::from_point_normal(point, &v1.cross(*v2))
    }

    /// Create a `Plane` which is facing along the X-Axis at the provided coordinate.
    pub fn with_x(x: f32) -> Self {
        Self::from_point_normal(&Vec3::new(x, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0))
    }

    /// Create a `Plane` which is facing along the Y-Axis at the provided coordinate.
    pub fn with_y(y: f32) -> Self {
        Self::from_point_normal(&Vec3::new(0.0, y, 0.0), &Vec3::new(0.0, 1.0, 0.0))
    }

    /// Create a `Plane` which is facing along the Z-Axis at the provided coordinate.
    pub fn with_z(z: f32) -> Self {
        Self::from_point_normal(&Vec3::new(0.0, 0.0, z), &Vec3::new(0.0, 0.0, 1.0))
    }

    /// f32his `Plane` normal
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    /// Normalized representation of this `Plane`
    pub fn normalize(&self) -> Self {
        let distance = self.normal.mag();
        Self {
            normal: self.normal / distance,
            bias: self.bias / distance,
        }
    }

    /// Returns the dot product of this `Plane` and a provided `Vec3`
    pub fn dot_point(&self, point: &Vec3) -> f32 {
        self.normal.x * point.x + self.normal.y * point.y + self.normal.z * point.z + self.bias
    }

    /// Returns the dot product of this `Plane` and a provided `Vec3`
    pub fn dot(&self, point: &Vec3) -> f32 {
        self.normal.x * point.x + self.normal.y * point.y + self.normal.z * point.z
    }

    /// Returns the dot product of this `Plane` with another `Plane`
    pub fn dot_plane(&self, plane: &Plane) -> f32 {
        self.normal.x * plane.normal.x
            + self.normal.y * plane.normal.y
            + self.normal.z * plane.normal.z
            + self.bias * plane.bias
    }

    /// Returns the intersection distance of the provided line given a point and direction, or `None` if none occurs.
    pub fn intersect_line(&self, point: &Vec3, direction: &Vec3) -> Option<f32> {
        let fv = self.dot(direction);
        let distance = self.dot_point(point) / fv;
        if fv.abs() > std::f32::MIN {
            Some(distance)
        } else {
            None
        }
    }

    /// Returns the intersection distance of the provided `Ray`, or `None` if none occurs.
    pub fn intersect_ray(&self, ray: &Ray) -> Option<f32> {
        self.intersect_line(&ray.origin, &ray.direction)
    }
}

/// A Ray represents and infinite half-line starting at `origin` and going in specified unit length `direction`.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    /// f32he origin point of the ray
    pub origin: Vec3,
    /// f32he normalized direction vector of the ray
    pub direction: Vec3,
}
impl Ray {
    /// Returns the distance along the ray which intersects with the provided `Plane`1
    pub fn intersect_plane(&self, plane: &Plane) -> Option<f32> {
        plane.intersect_ray(self)
    }

    /// Returns a `Vec3` along the ray at a distance `t` from it's origin.
    pub fn at_distance(&self, z: f32) -> Vec3 {
        self.origin - (self.direction * z)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use approx::{assert_ulps_eq, relative_eq};

    #[test]
    #[allow(clippy::mistyped_literal_suffixes)]
    fn ray_intersect_plane() {
        let plane = Plane::<f32>::with_z(0.0);

        let ray = Ray {
            origin: Vec3::new(0.020_277_506, -0.033_236_53, 51.794),
            direction: Vec3::new(0.179_559_51, -0.294_313_04, -0.938_689_65),
        };
        let distance = ray.intersect_plane(&plane).unwrap();
        let point = ray.at_distance(distance);
        assert_ulps_eq!(point, Vec3::new(9.927_818, -16.272_524, 0.0));

        let ray = Ray {
            origin: Vec3::new(-0.003_106_177, 0.034_074_64, 0.799_999_95),
            direction: Vec3::new(-0.029_389_05, 0.322_396_73, -0.946_148_3),
        };
        let distance = ray.intersect_plane(&plane).unwrap();
        let point = ray.at_distance(distance);
        assert_ulps_eq!(point, Vec3::new(-0.027_955_6, 0.306_671_83, 0.0));
    }

    #[test]
    fn at_distance() {
        relative_eq!(
            Ray {
                origin: Vec3::new(0.020_277_506, -0.033_236_53, 51.794),
                direction: Vec3::new(0.179_559_51, -0.294_313_04, -0.938_689_65),
            }
            .at_distance(5.0),
            Vec3::new(0.918_075_1, -1.504_801_8, 47.100_55)
        );
    }
}
