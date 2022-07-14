use pyo3::prelude::*;
use rustrict::CensorStr;
use rustrict::Type;

#[pyfunction]
fn is_any(text: String) -> PyResult<bool> {
	Ok(text.is(Type::ANY))
}

#[pyfunction]
fn is_inappropriate(text: String) -> PyResult<bool> {
	Ok(text.is_inappropriate())
}

#[pyfunction]
fn censor(text: String) -> PyResult<String> {
	Ok(text.censor())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustrict(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(is_any, m)?)?;
	m.add_function(wrap_pyfunction!(is_inappropriate, m)?)?;
	m.add_function(wrap_pyfunction!(censor, m)?)?;
	Ok(())
}
