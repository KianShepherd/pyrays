use serde::{Deserialize, Serialize};
use std::intrinsics::{fadd_fast, fdiv_fast, fmaf32, fmul_fast, fsub_fast, maxnumf32, minnumf32};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn clamp(val: f32, min: f32, max: f32) -> f32 {
    maxnumf32(minnumf32(val, max), min)
}

impl Vec3 {
    pub fn new(_x: f32, _y: f32, _z: f32) -> Vec3 {
        Vec3 {
            x: _x,
            y: _y,
            z: _z,
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.x
    }
    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.y
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn get_idx(&self, idx: usize) -> f32 {
        if idx == 0 {
            self.x
        } else if idx == 1 {
            self.y
        } else {
            self.z
        }
    }

    pub fn set_idx(&mut self, idx: usize, val: f32) {
        if idx == 0 {
            self.x = val;
        } else if idx == 1 {
            self.y = val;
        } else {
            self.z = val;
        }
    }

    #[inline(always)]
    pub fn dot(&self, v: &Vec3) -> f32 {
        unsafe { fmaf32(self.x, v.x, fmaf32(self.y, v.y, fmul_fast(self.z, v.z))) }
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn cross(&self, v: &Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fsub_fast(fmul_fast(self.y, v.z), fmul_fast(self.z, v.y)),
                y: fsub_fast(fmul_fast(self.z, v.x), fmul_fast(self.x, v.z)),
                z: fsub_fast(fmul_fast(self.x, v.y), fmul_fast(self.y, v.x)),
            }
        }
    }
    #[inline(always)]
    pub fn length_squared(&self) -> f32 {
        unsafe {
            fmaf32(
                self.x,
                self.x,
                fmaf32(self.y, self.y, fmul_fast(self.z, self.z)),
            )
        }
    }
    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    #[inline(always)]
    pub fn unit_vector(&self) -> Vec3 {
        unsafe { self * (fdiv_fast(1.0, self.length())) }
    }

    #[allow(dead_code)]
    pub fn to_string(self, samples_per_pixel: usize) -> String {
        let scale = 1.0 / samples_per_pixel as f32;
        let r = (256.0 * clamp((self.x * scale).sqrt(), 0.0, 0.999)) as u8;
        let g = (256.0 * clamp((self.y * scale).sqrt(), 0.0, 0.999)) as u8;
        let b = (256.0 * clamp((self.z * scale).sqrt(), 0.0, 0.999)) as u8;
        format!("{} {} {}", r, g, b)
    }

    pub fn to_rgb(self, samples_per_pixel: usize) -> Vec<u8> {
        unsafe {
            let scale = fdiv_fast(1.0, samples_per_pixel as f32);
            let r = fmul_fast(256.0, clamp(fmul_fast(self.x, scale).sqrt(), 0.0, 0.999)) as u8;
            let g = fmul_fast(256.0, clamp(fmul_fast(self.y, scale).sqrt(), 0.0, 0.999)) as u8;
            let b = fmul_fast(256.0, clamp(fmul_fast(self.z, scale).sqrt(), 0.0, 0.999)) as u8;

            vec![r, g, b]
        }
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, rhs: &Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fadd_fast(self.x, rhs.x),
                y: fadd_fast(self.y, rhs.y),
                z: fadd_fast(self.z, rhs.z),
            }
        }
    }
}

impl std::ops::Add<f32> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, rhs: f32) -> Vec3 {
        unsafe {
            Vec3 {
                x: fadd_fast(self.x, rhs),
                y: fadd_fast(self.y, rhs),
                z: fadd_fast(self.z, rhs),
            }
        }
    }
}

impl std::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, rhs: &Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fsub_fast(self.x, rhs.x),
                y: fsub_fast(self.y, rhs.y),
                z: fsub_fast(self.z, rhs.z),
            }
        }
    }
}

impl std::ops::Sub<f32> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, rhs: f32) -> Vec3 {
        unsafe {
            Vec3 {
                x: fsub_fast(self.x, rhs),
                y: fsub_fast(self.y, rhs),
                z: fsub_fast(self.z, rhs),
            }
        }
    }
}

impl std::ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: &Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fmul_fast(self.x, rhs.x),
                y: fmul_fast(self.y, rhs.y),
                z: fmul_fast(self.z, rhs.z),
            }
        }
    }
}

impl std::ops::Mul<&Vec3> for f32 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: &Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fmul_fast(self, rhs.x()),
                y: fmul_fast(self, rhs.y()),
                z: fmul_fast(self, rhs.z()),
            }
        }
    }
}

impl std::ops::Mul<f32> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Vec3 {
        unsafe {
            Vec3 {
                x: fmul_fast(self.x, rhs),
                y: fmul_fast(self.y, rhs),
                z: fmul_fast(self.z, rhs),
            }
        }
    }
}

impl std::ops::Div<&Vec3> for f32 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, rhs: &Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fdiv_fast(self, rhs.x()),
                y: fdiv_fast(self, rhs.y()),
                z: fdiv_fast(self, rhs.z()),
            }
        }
    }
}

impl std::ops::Div<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, rhs: &Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fdiv_fast(self.x, rhs.x),
                y: fdiv_fast(self.y, rhs.y),
                z: fdiv_fast(self.z, rhs.z),
            }
        }
    }
}
impl std::ops::Div<f32> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, rhs: f32) -> Vec3 {
        unsafe {
            Vec3 {
                x: fdiv_fast(self.x, rhs),
                y: fdiv_fast(self.y, rhs),
                z: fdiv_fast(self.z, rhs),
            }
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, rhs: Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fadd_fast(self.x, rhs.x),
                y: fadd_fast(self.y, rhs.y),
                z: fadd_fast(self.z, rhs.z),
            }
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, rhs: Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fsub_fast(self.x, rhs.x),
                y: fsub_fast(self.y, rhs.y),
                z: fsub_fast(self.z, rhs.z),
            }
        }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fmul_fast(self.x, rhs.x),
                y: fmul_fast(self.y, rhs.y),
                z: fmul_fast(self.z, rhs.z),
            }
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fmul_fast(self, rhs.x()),
                y: fmul_fast(self, rhs.y()),
                z: fmul_fast(self, rhs.z()),
            }
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Vec3 {
        unsafe {
            Vec3 {
                x: fmul_fast(self.x, rhs),
                y: fmul_fast(self.y, rhs),
                z: fmul_fast(self.z, rhs),
            }
        }
    }
}

impl std::ops::Div<Vec3> for f32 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, rhs: Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fdiv_fast(self, rhs.x()),
                y: fdiv_fast(self, rhs.y()),
                z: fdiv_fast(self, rhs.z()),
            }
        }
    }
}

impl std::ops::Div for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, rhs: Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                x: fdiv_fast(self.x, rhs.x),
                y: fdiv_fast(self.y, rhs.y),
                z: fdiv_fast(self.z, rhs.z),
            }
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() -> Result<(), String> {
        assert_eq!(clamp(1.5, 0.0, 1.0), 1.0);
        assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
        assert_eq!(clamp(-1.5, 0.0, 1.0), 0.0);
        Ok(())
    }

    #[test]
    fn test_getters() -> Result<(), String> {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.to_string(1), String::from("255 255 255"));
        Ok(())
    }

    #[test]
    fn test_math() -> Result<(), String> {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let f = 0.5;

        assert_eq!(f * v, Vec3::new(0.5, 0.5, 0.5));
        assert_eq!(0.5 / v, Vec3::new(0.5, 0.5, 0.5));
        assert_eq!(v / Vec3::new(2.0, 2.0, 2.0), Vec3::new(0.5, 0.5, 0.5));
        assert_eq!(-v, Vec3::new(-1.0, -1.0, -1.0));
        Ok(())
    }
}
