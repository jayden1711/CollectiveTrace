pub mod parser;

use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use std::path::Path;

#[pyfunction]
fn parse_trace_summary(path: String) -> PyResult<(usize, f64)> {
    let summary = parser::parse_file(Path::new(&path)).map_err(|e| PyIOError::new_err(e.to_string()))?;
    Ok((summary.event_count, summary.total_duration_us))
}

#[pymodule]
fn _core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_trace_summary, m)?)?;
    Ok(())
}
