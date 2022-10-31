use pyo3::{PyResult, PyModule, Python};
use numpy::ndarray::{ArrayD};
use numpy::{IntoPyArray};

/// Computes the gower matrix of a given numpy array
#[pyfunction]
fn get_gower(arr: ArrayD) -> PyResult<String> {
    
    fn binary() {

    }

    fn jaccard() {

    }

    fn dice() {

    }

    fn manhattan() {

    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn gower(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_gower, m)?)?;
    Ok(())
}