use rand::Rng;

use crate::rtweekend::{clamp, random_double, random_double_range};


#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vec3 {
    pub e: [f64; 3]
}

pub type Colour = Vec3;
pub type Point3 = Vec3;


impl Vec3 {
    pub fn new() -> Vec3 {

        return Vec3 {
            e: [0.0, 0.0, 0.0]
        };
    }

    pub fn random() -> Vec3 {
        return Vec3::fill(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        return Vec3::fill(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max));
    }

    pub fn fill<T: Into<f64>>(x: T, y: T, z: T) -> Vec3 {
        return Vec3 {
            e: [x.into(), y.into(), z.into()]
        };
    }

    pub fn x(&self) -> f64 {
        return self.e[0];
    }

    pub fn y(&self) -> f64 {
        return self.e[1];
    }

    pub fn z(&self) -> f64 {
        return self.e[2];
    }

    pub fn len_squared(&self) -> f64 {
        return (self.e[0] * self.e[0]) + (self.e[1] * self.e[1]) + (self.e[2] * self.e[2]);
        // Parentheses are for clarity, I know how BEDMAS works
    }

    pub fn len(&self) -> f64 {
        return self.len_squared().sqrt();
    }

    pub fn debug_format(&self) -> String {
        return format!("{} {} {}", self.e[0], self.e[1], self.e[2]);
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        return 
                self.e[0] * other.e[0] +
                self.e[1] * other.e[1] +
                self.e[2] * other.e[2];
            
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        return Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0]
            ]
        };
    }
    
    pub fn unit_vector(&self) -> Vec3{
        return *self / self.len();
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        return Vec3::fill(0.5, 0.5, 0.5)
    }
    /* pub fn format_colour(&self) -> String {
        let ir = (255.999 * self.e[0]) as i32;
        let ig = (255.999 * self.e[1]) as i32;
        let ib = (255.999 * self.e[2]) as i32;

        return format!("{} {} {}\n", ir, ig, ib);
    } */

    pub fn format_colour(&self, samples_per_pixel: i32) -> String {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // Divide Colour by number of samples 
        let scale = 1.0 / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
        let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
        let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

        return format!("{} {} {}\n", ir, ig, ib);
    }

    
}



impl std::ops::AddAssign for Vec3 {

    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl std::ops::MulAssign<f64> for Vec3 {

    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}


impl std::ops::DivAssign<f64> for Vec3 {

    fn div_assign(&mut self, rhs: f64) {
        self.e[0] *= 1.0 / rhs;
        self.e[1] *= 1.0 / rhs;
        self.e[2] *= 1.0 / rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        let x = self.e[0] * 1.0 / rhs;
        let y = self.e[1] * 1.0 / rhs;
        let z = self.e[2] * 1.0 / rhs;

        return Vec3::fill(x, y, z);
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3 ) -> Vec3 {
        return Vec3  { 
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]
            ]
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        return Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]
            ]
        }
    }
}


impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2]
            ]
        }
    }

}

/* impl core::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl core::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [self * v.e[0], self * v.e[1], self * v.e[2]],
        }
    }
}

impl core::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
} */

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        return Vec3 {
            e: [
            v.e[0] * self,
            v.e[1] * self,
            v.e[2] * self,
            ]
        };
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        return Vec3 {
            e: [
            self.e[0] * t,
            self.e[1] * t,
            self.e[2] * t,
            ]
        };
    }
}


impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3 {
            e: [
                -self.e[0],
                -self.e[1],
                -self.e[2]
            ]
        }
    }
}


/* impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: [
            self.e[0] / other,
            self.e[1] / other,
            self.e[2] / other,
            ]
        }
    }
} */

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn vec3_addition() {
        let vec1 = Vec3::fill(1, 2, 3);
        let vec2 = Vec3::fill(2, 5, 9);

        assert_eq!(vec1 + vec2, Vec3::fill(3, 7, 12))
    }

    #[test]
    fn vec3_subtraction() {
        let vec1 = Vec3::fill(6, 8, 9);
        let vec2 = Vec3::fill(4, 2, 7);

        assert_eq!(vec1 - vec2, Vec3::fill(2, 6, 2));
    }

    #[test]
    fn vec3_multiplication() {
        let vec1 = Vec3::fill(2, 4, 8);
        let vec2 = Vec3::fill(2, 2, 2);

        assert_eq!(vec1 * vec2, 2.0 * vec1);
    }


    #[test]
    fn vec3_division_by_f64() {
        let vec1 = Vec3::fill(10, 16, 24);

        assert_eq!(vec1 / 2.0 , Vec3::fill(5, 8, 12))
    }

    #[test]
    fn vec3_unit_vector() {
        let vec1 = Vec3::fill(3, 4, 5);
        let unit_vector = vec1.unit_vector();
        assert_eq!(Vec3::fill(0.4242640687119285, 0.565685424949238, 0.7071067811865475), unit_vector);
    }
    /*
    FEATURE NO LONGER REQUIRED
    #[test]
    fn vec3_division_by_vec3() {
        let vec1 = Vec3::fill(2, 2, 2);
        let vec2 = Vec3::fill(10, 16, 24);

        assert_eq!(vec2 / vec1 , Vec3::fill(5, 8, 12))
    } */
}