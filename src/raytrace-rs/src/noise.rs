use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Fbm, MultiFractal, NoiseFn, Seedable};
use rand::Rng;

pub struct Noise {
    pub noise_map: Vec<f32>,
}

#[derive(Debug, Clone, Copy)]
struct RainEroder {
    idx: usize,
    d_x: i64,
    d_y: i64,
    height: f32,
}

impl RainEroder {
    fn new(x: i64, y: i64, d_x: i64, d_y: i64, r1: usize, height_map: &Vec<f32>) -> Self {
        let idx = ((y + d_y) as usize * r1) + (x + d_x) as usize;
        let height = height_map[idx];
        Self {
            idx,
            d_x,
            d_y,
            height,
        }
    }
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
        rain_factor: f64,
        drops_per_point: usize,
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

        let r1 = resolution + 1;
        for _ in 0..octaves {
            let fbm = Fbm::new()
                .set_seed(seed_value)
                .set_octaves(0)
                .set_frequency(freq as f64);
            PlaneMapBuilder::new(&fbm)
                .set_size(resolution, resolution)
                .build();
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
                        * (1.0
                            / (1.0
                                + (if erosion_factor > 0.0 {
                                    erosion_factor
                                } else {
                                    0.0
                                } * full_derivative_map[i])
                                    .abs()))
                        .abs())
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

        if rain_factor > 0.0 {
            for i in 0..noise_map.len() {
                noise_map[i] = noise_map[i] * 10.0;
            }
            let min_diff = 0.001;
            let droplets = (r1 * r1) * drops_per_point;
            let mut rng = rand::thread_rng();
            for _ in 0..droplets {
                let mut d_x: i64 = rng.gen_range(1..(r1 as i64 - 1));
                let mut d_y: i64 = rng.gen_range(1..(r1 as i64 - 1));
                let mut itrs = 0;

                loop {
                    let i = (d_y as usize * r1) + (d_x) as usize;
                    let d = noise_map[i];
                    let direction = vec![
                        RainEroder::new(d_x, d_y, -1, 0, r1, &noise_map),
                        RainEroder::new(d_x, d_y, 1, 0, r1, &noise_map),
                        RainEroder::new(d_x, d_y, 0, -1, r1, &noise_map),
                        RainEroder::new(d_x, d_y, 0, 1, r1, &noise_map),
                    ]
                    .iter()
                    .fold(None::<RainEroder>, |acc, x| {
                        if let Some(a) = acc {
                            if x.height < a.height {
                                Some(*x)
                            } else {
                                acc
                            }
                        } else {
                            if x.height < d {
                                Some(*x)
                            } else {
                                None
                            }
                        }
                    });
                    if let Some(dir) = direction {
                        if dir.height < d {
                            let mut diff = d - dir.height;
                            if diff < min_diff {
                                break;
                            }
                            diff *= 1.0 / rain_factor as f32;
                            noise_map[i] -= diff;
                            noise_map[dir.idx] += diff;
                            d_x += dir.d_x;
                            d_y += dir.d_y;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }

                    if d_y == 0
                        || d_x == 0
                        || d_x == r1 as i64 - 1
                        || d_y == r1 as i64 - 1
                        || itrs > r1
                    {
                        break;
                    }
                    itrs += 1;
                }
            }

            highest = f64::MIN;
            lowest = f64::MAX;
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
        }

        Noise { noise_map }
    }
}
