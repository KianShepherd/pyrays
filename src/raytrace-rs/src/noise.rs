use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Fbm, MultiFractal, NoiseFn, Seedable};

pub struct Noise {
    pub noise_map: Vec<f32>,
}

impl Noise {
    pub fn new(
        resolution: usize,
        octaves: usize,
        frequency: f32,
        lacunarity: f32,
        seed_value: u32,
        persistence: f32,
    ) -> Noise {
        let mut noise_map = vec![];
        let fbm = Fbm::new()
            .set_seed(seed_value)
            .set_octaves(octaves)
            .set_frequency(frequency as f64)
            .set_lacunarity(lacunarity as f64)
            .set_persistence(persistence as f64);
        PlaneMapBuilder::new(&fbm)
            .set_size(resolution, resolution)
            .build();
        let mut highest = f64::MIN;
        let mut lowest = f64::MAX;
        let r1 = resolution + 1;
        for i in 0..r1 {
            for j in 0..r1 {
                let hightval = fbm.get([j as f64, i as f64]);
                noise_map.push(hightval as f32);
                if hightval > highest {
                    highest = hightval;
                }
                if hightval < lowest {
                    lowest = hightval;
                }
            }
        }

        if lowest < 0.0 {
            lowest = -lowest;
        }

        for i in 0..noise_map.len() {
            noise_map[i] = (noise_map[i] + lowest as f32) / (highest + lowest) as f32;
        }

        Noise { noise_map }
    }
}
