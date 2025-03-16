// Main library functionality
pub fn sum_as_string(a: usize, b: usize) -> String {
    (a + b).to_string()
}

// PyO3 bindings only compiled when the python-bindings feature is enabled
#[cfg(feature = "python-bindings")]
mod python {
    use super::sum_as_string;
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction(name = "sum_as_string")]
    fn sum_as_string_py(a: usize, b: usize) -> PyResult<String> {
        Ok(sum_as_string(a, b))
    }

    #[pymodule]
    fn lr_percolation(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(sum_as_string_py, m)?)?;
        Ok(())
    }
}
