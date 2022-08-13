use ron::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RaytracerScene {
    pub multithreading: bool,
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: usize,
    pub max_depth: i32,
    pub v_fov: f64,
    pub aperture: f64,
    pub focal_distance: f64,
    pub camera_pos: Vec<f64>,
    pub camera_dir: Vec<f64>,
    pub camera_up: Vec<f64>,
    pub objects: Vec<RonObject>,
    pub lights: Vec<Vec<f64>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RonObject {
    pub objtype: String,
    pub vectors: Vec<Vec<f64>>,
    pub scalars: Vec<f64>,
    pub material: Vec<String>,
}

#[allow(dead_code)]
impl RaytracerScene {
    pub fn to_ron(&self) -> String {
        let pretty = PrettyConfig::new();

        to_string_pretty(&self, pretty).expect("serialization failed")
    }

    pub fn from_ron(ron_string: String) -> RaytracerScene {
        from_str(&ron_string).expect("deserialization failed")
    }
}
