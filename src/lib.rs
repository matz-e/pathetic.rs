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

    m.add_class::<Rhomboid>()?;
    m.add_class::<Sphere>()?;
    m.add_class::<Triangle>()?;

    m.add_class::<Camera>()?;
    m.add_class::<Lens>()?;

    #[pyfn(m, "render")]
    fn render(
        _py: Python,
        camera: Camera,
        objects: &PyList,
        filename: &str,
        dpi: u32,
        samples: Option<usize>,
        bounces: Option<usize>,
    ) {
        let mut scene = Scene::new(camera, samples, bounces);
        for obj in objects {
            let rhomboid: Result<Rhomboid, _> = obj.extract();
            if let Ok(r) = rhomboid {
                scene.add(r);
                continue;
            }

            let sphere: Result<Sphere, _> = obj.extract();
            if let Ok(s) = sphere {
                scene.add(s);
                continue;
            }

            let triangle: Result<Triangle, _> = obj.extract();
            if let Ok(r) = triangle {
                scene.add(r);
            }
        }
        scene.render(filename, dpi).unwrap();
    }

    Ok(())
}
