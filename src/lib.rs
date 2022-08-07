use pyo3::prelude::*;
use raytrace_rs;

#[pyfunction]
fn create_scene(scene_ron: String) -> Vec<Vec<u8>> {
    raytrace_rs::create_image(scene_ron)
}

#[pymodule]
fn pyrays_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_scene, m)?)?;
    Ok(())
}
