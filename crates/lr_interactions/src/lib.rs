use ::percolation::norms;
use ::percolation::percolation;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
enum Norm {
    L1,
    L2,
    LInf,
}

#[pyclass]
struct Observables {
    #[pyo3(get)]
    average_size: f64,
    #[pyo3(get)]
    size_spread: f64,
}

impl Observables {
    fn from(o: percolation::Observables) -> Self {
        Observables {
            average_size: o.average_size,
            size_spread: o.size_spread,
        }
    }
}

#[pyfunction(name = "simulate")]
fn simulate_perc(
    norm: Norm,
    l: usize,
    alpha: f64,
    beta: f64,
    n_samples: u64,
    seed: u64,
) -> PyResult<Vec<Observables>> {
    let res = match norm {
        Norm::L1 => percolation::simulate(norms::Norm::L1, l, alpha, beta, n_samples, seed),
        Norm::L2 => percolation::simulate(norms::Norm::L1, l, alpha, beta, n_samples, seed),
        Norm::LInf => percolation::simulate(norms::Norm::L1, l, alpha, beta, n_samples, seed),
    };
    Ok(res.into_iter().map(Observables::from).collect())
}

fn register_percolation(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let percolation = PyModule::new(parent_module.py(), "percolation")?;
    percolation.add_class::<Observables>()?;
    percolation.add_class::<Norm>()?;
    percolation.add_function(wrap_pyfunction!(simulate_perc, &percolation)?)?;
    parent_module.add_submodule(&percolation)
}

#[pyclass]
struct Point2D {
    #[pyo3(get)]
    x: i32,
    #[pyo3(get)]
    y: i32,
}

impl Point2D {
    fn from(p: lerw::Point2D) -> Self {
        Point2D { x: p.x, y: p.y }
    }
}

#[pyfunction(name = "simulate")]
fn simulate_lerw(len: usize, seed: u64) -> PyResult<Vec<Point2D>> {
    let res = lerw::simulate(len, seed);
    Ok(res.into_iter().map(Point2D::from).collect())
}

fn register_lerw(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let lerw = PyModule::new(parent_module.py(), "random_walks")?;
    lerw.add_class::<Point2D>()?;
    lerw.add_function(wrap_pyfunction!(simulate_lerw, &lerw)?)?;
    parent_module.add_submodule(&lerw)
}

#[pymodule]
fn lr_interactions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_percolation(m)?;
    register_lerw(m)?;
    Ok(())
}
