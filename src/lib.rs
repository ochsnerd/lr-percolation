pub use percolation::*;

pub mod norms;
pub mod percolation;

#[cfg(feature = "python-bindings")]
mod python {
    use crate::norms;
    use crate::percolation;
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

    #[pyfunction]
    fn simulate(
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

    #[pymodule]
    fn lr_percolation(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<Observables>()?;
        m.add_class::<Norm>()?;
        m.add_function(wrap_pyfunction!(simulate, m)?)?;
        Ok(())
    }
}
