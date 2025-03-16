pub use percolation::*;

mod percolation;

// PyO3 bindings only compiled when the python-bindings feature is enabled
#[cfg(feature = "python-bindings")]
mod python {
    use crate::percolation;
    use pyo3::prelude::*;

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

    #[pyfunction]
    fn simulate_l1(
        l: usize,
        sigma: f64,
        beta: f64,
        n_samples: u64,
        seed: u64,
    ) -> PyResult<Vec<Observables>> {
        let res = percolation::simulate(percolation::Norm::L1, l, sigma, beta, n_samples, seed);
        Ok(res.into_iter().map(|o| Observables::from(o)).collect())
    }

    #[pyfunction]
    fn simulate_linf(
        l: usize,
        sigma: f64,
        beta: f64,
        n_samples: u64,
        seed: u64,
    ) -> PyResult<Vec<Observables>> {
        let res = percolation::simulate(percolation::Norm::LInf, l, sigma, beta, n_samples, seed);
        Ok(res.into_iter().map(|o| Observables::from(o)).collect())
    }

    #[pymodule]
    fn lr_percolation(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<Observables>()?;
        m.add_function(wrap_pyfunction!(simulate_linf, m)?)?;
        m.add_function(wrap_pyfunction!(simulate_l1, m)?)?;
        Ok(())
    }
}
