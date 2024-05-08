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
        erosion_factor: f64,
    ) -> Noise {
        let mut noise_map = vec![0.0; (resolution + 1) * (resolution + 1)];
        let mut full_derivative_map = vec![0.0; (resolution + 1) * (resolution + 1)];
        let mut freq = frequency;
        let mut persis = 1.0;
        let mut highest = f64::MIN;
        let mut lowest = f64::MAX;

        for _ in 0..octaves {
            let fbm = Fbm::new()
                .set_seed(seed_value)
                .set_octaves(0)
                .set_frequency(freq as f64);
            PlaneMapBuilder::new(&fbm)
                .set_size(resolution, resolution)
                .build();
            let r1 = resolution + 1;
            let mut derivative_map = vec![0.0; (resolution + 1) * (resolution + 1)];
            let mut d_highest = f64::MIN;
            let mut d_lowest = f64::MAX;
            for i in 0..r1 {
                for j in 0..r1 {
                    let d_x1 =
                        (fbm.get([j as f64, i as f64]) - fbm.get([j as f64 + 0.1, i as f64])).abs()
                            / 0.1;
                    let d_x2 =
                        (fbm.get([j as f64 - 0.1, i as f64]) - fbm.get([j as f64, i as f64])).abs()
                            / 0.1;
                    let d_y1 =
                        (fbm.get([j as f64, i as f64]) - fbm.get([j as f64, i as f64 + 0.1])).abs()
                            / 0.1;
                    let d_y2 =
                        (fbm.get([j as f64, i as f64 - 0.1]) - fbm.get([j as f64, i as f64])).abs()
                            / 0.1;
                    derivative_map[(i * r1) + j] +=
                        (((d_x1 + d_x2) / 2.0).abs() + ((d_y1 + d_y2) / 2.0).abs()) / 2.0;
                    let hightval = derivative_map[(i * r1) + j] as f64;
                    if hightval > d_highest {
                        d_highest = hightval;
                    }
                    if hightval < d_lowest {
                        d_lowest = hightval;
                    }
                }
            }
            d_lowest = -d_lowest;
            for i in 0..r1 {
                for j in 0..r1 {
                    derivative_map[(i * r1) + j] =
                        (d_lowest + (derivative_map[(i * r1) + j])) / (d_highest + d_lowest);
                }
            }

            for i in 0..full_derivative_map.len() {
                full_derivative_map[i] = full_derivative_map[i] + derivative_map[i];
            }

            for i in 0..r1 {
                for j in 0..r1 {
                    noise_map[(i * r1) + j] += (fbm.get([j as f64, i as f64])
                        * persis as f64
                        * (1.0 / (1.0 + (erosion_factor * full_derivative_map[(i * r1) + j]))))
                        as f32;
                }
            }
            freq *= lacunarity;
            persis *= persistence;
        }
        for i in 0..(noise_map.len()) {
            let hightval = noise_map[i] as f64;
            if hightval > highest {
                highest = hightval;
            }
            if hightval < lowest {
                lowest = hightval;
            }
        }

        lowest = -lowest;

        for i in 0..noise_map.len() {
            noise_map[i] = (noise_map[i] + lowest as f32) / (highest + lowest) as f32;
        }

        Noise { noise_map }
    }
}
