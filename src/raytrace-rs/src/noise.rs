use std::intrinsics::powf64;

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
        let sqrt_p2 = {
            let p2: f64 = 0.5;
            let p2_sqrd: f64 = p2.exp2();
            (p2_sqrd + p2_sqrd).sqrt()
        };

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
            let mut layer_map = vec![0.0; (resolution + 1) * (resolution + 1)];
            let mut d_highest = f64::MIN;
            let mut d_lowest = f64::MAX;
            let mut l_highest = f64::MIN;
            let mut l_lowest = f64::MAX;
            for i in 0..r1 {
                for j in 0..r1 {
                    let d_x1 =
                        (fbm.get([j as f64, i as f64]) - fbm.get([j as f64 + 0.5, i as f64])).abs()
                            / 0.5;
                    let d_x2 =
                        (fbm.get([j as f64 - 0.5, i as f64]) - fbm.get([j as f64, i as f64])).abs()
                            / 0.5;
                    let d_y1 =
                        (fbm.get([j as f64, i as f64]) - fbm.get([j as f64, i as f64 + 0.5])).abs()
                            / 0.5;
                    let d_y2 =
                        (fbm.get([j as f64, i as f64 - 0.5]) - fbm.get([j as f64, i as f64])).abs()
                            / 0.5;
                    let d_z1 = (fbm.get([j as f64 - 0.5, i as f64 - 0.5])
                        - fbm.get([j as f64, i as f64]))
                    .abs()
                        / sqrt_p2;
                    let d_z2 = (fbm.get([j as f64, i as f64])
                        - fbm.get([j as f64 + 0.5, i as f64 + 0.5]))
                    .abs()
                        / sqrt_p2;
                    let d_w1 = (fbm.get([j as f64 - 0.5, i as f64 + 0.5])
                        - fbm.get([j as f64, i as f64]))
                    .abs()
                        / sqrt_p2;
                    let d_w2 = (fbm.get([j as f64, i as f64])
                        - fbm.get([j as f64 + 0.5, i as f64 - 0.5]))
                    .abs()
                        / sqrt_p2;
                    derivative_map[(i * r1) + j] = ((((d_x1 + d_x2) / 2.0).abs()
                        + ((d_y1 + d_y2) / 2.0).abs()
                        + ((d_z1 + d_z2) / 2.0).abs()
                        + ((d_w1 + d_w2) / 2.0).abs())
                        / 4.0)
                        .abs();
                    let hightval = derivative_map[(i * r1) + j] as f64;
                    if hightval > d_highest {
                        d_highest = hightval;
                    }
                    if hightval < d_lowest {
                        d_lowest = hightval;
                    }
                    layer_map[(i * r1) + j] = fbm.get([j as f64, i as f64]);
                    let l_hightval = layer_map[(i * r1) + j] as f64;
                    if l_hightval > l_highest {
                        l_highest = l_hightval;
                    }
                    if l_hightval < l_lowest {
                        l_lowest = l_hightval;
                    }
                }
            }
            d_lowest = -d_lowest;
            l_lowest = -l_lowest;

            for i in 0..full_derivative_map.len() {
                full_derivative_map[i] = (full_derivative_map[i]
                    + (((d_lowest + (derivative_map[i])) / (d_highest + d_lowest)).abs())
                        * persis as f64)
                    .abs();
                noise_map[i] += (((l_lowest + layer_map[i]) / (l_lowest + l_highest))
                    * (persis as f64
                        * (1.0 / (1.0 + (erosion_factor * full_derivative_map[i]).abs())).abs())
                    .abs())
                .abs() as f32;
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
