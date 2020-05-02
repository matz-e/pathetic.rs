use pyo3::prelude::*;
use pyo3::types::PyList;

mod scene;
mod things;

use scene::*;
use things::*;

#[pymodule]
fn pathetic(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Ray>()?;
    m.add_class::<Color>()?;
    m.add_class::<Material>()?;

    m.add_class::<Sphere>()?;
    m.add_class::<Rhomboid>()?;

    m.add_class::<Camera>()?;

    #[pyfn(m, "render")]
    fn render(
        _py: Python,
        camera: Camera,
        objects: &PyList,
        filename: &str,
        dpi: u32,
        samples: usize,
        bounces: usize,
    ) {
        let mut scene = Scene::new(camera);
        for obj in objects {
            let s: Result<Sphere, _> = obj.extract();
            if s.is_ok() {
                scene.add(s.unwrap());
                continue;
            }

            let r: Result<Rhomboid, _> = obj.extract();
            if r.is_ok() {
                scene.add(r.unwrap());
            }
        }
        scene.render(filename, dpi, samples, bounces).unwrap();
    }

    Ok(())
}
