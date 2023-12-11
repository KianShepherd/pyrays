use crate::random_f32;
use glam::Vec3A;

pub struct ColourMap {
    colour_vec: Vec<ColourData>,
    default_colour: Vec3A,
    fuzz: f32,
}

pub struct ColourData {
    pub cutoff: f32,
    pub colour: Vec3A,
}

impl ColourMap {
    #[allow(dead_code)]
    pub fn new(colour_data: Vec<ColourData>, default_colour_: Vec3A, fuzz: f32) -> Self {
        ColourMap {
            colour_vec: colour_data,
            default_colour: default_colour_,
            fuzz,
        }
    }

    pub fn to_colour(&self, value: f32) -> Vec3A {
        let val = (random_f32(-self.fuzz, self.fuzz) + value)
            .min(1.0)
            .max(0.0);
        for i in 0..self.colour_vec.len() {
            if val > self.colour_vec[i].cutoff {
                return self.colour_vec[i].colour;
            }
        }
        return self.default_colour;
    }
}
