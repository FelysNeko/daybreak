use bootstrap::Parser;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::fs::read_to_string;
use std::path::PathBuf;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn generate(path: PathBuf, v: bool) -> PyResult<(String, Vec<String>)> {
    let input = read_to_string(path)?;
    match Parser::new(input, v).generate() {
        Some(result) => Ok(result),
        None => Err(PyException::new_err("Parsing failed due to invalid syntax"))
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn interface(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    Ok(())
}
