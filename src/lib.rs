use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn compute_sum(a: i64, b: i64) -> PyResult<i64> {
    Ok((a + b))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_sum, m)?)?;
    Ok(())
}
