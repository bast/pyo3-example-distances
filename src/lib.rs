use rayon::prelude::*;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn distance_squared(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    dx * dx + dy * dy
}

// this is not an efficient algorithm to find distances to nearest neighbors
// but we use it here to keep things relatively simple
fn nearest_distance(reference_point: (f64, f64), other_points: &[(f64, f64)]) -> f64 {
    let mut min_distance = std::f64::MAX;

    for &point in other_points {
        let d2 = distance_squared(reference_point, point);
        // make sure we don't locate the point itself otherwise minimum
        // distance is zero
        if d2 > std::f64::EPSILON {
            min_distance = min_distance.min(d2);
        }
    }

    min_distance.sqrt()
}

/// some docstring in here
#[pyfunction]
pub fn nearest_distances(points: Vec<(f64, f64)>) -> Vec<f64> {
    println!("hello from rust!");

    points
        .par_iter()
        .map(|&p| nearest_distance(p, &points))
        .collect()
}

#[pymodule]
fn distances(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_function(wrap_pyfunction!(nearest_distances, m)?)?;

    Ok(())
}
