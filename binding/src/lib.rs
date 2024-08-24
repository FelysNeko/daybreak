use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pyclass(get_all)]
pub struct PegInfo {
    pub json: String,
    pub node: Vec<String>,
}

#[pyfunction]
fn parse(input: String, v: bool) -> PyResult<PegInfo> {
    let info = bootstrap::parse(input, v)
        .map_err(|e| { PyException::new_err(e) })?;
    Ok(PegInfo {
        json: info.json,
        node: info.node,
    })
}

#[pymodule]
fn binding(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_class::<PegInfo>()?;
    Ok(())
}
