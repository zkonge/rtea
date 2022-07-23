use pyo3::{exceptions::PyValueError, prelude::*, types::PyBytes, wrap_pyfunction};

mod block;
mod tea;

#[pyfunction]
fn is_debug() -> bool {
    cfg!(debug_assertions)
}

/// tea16_encrypt(text:bytes, key:bytes) -> bytes
/// --
///
/// Encrypt text with key using 16-rounds TEA
#[pyfunction]
fn tea16_encrypt<'a>(py: Python, text: &'a [u8], key: &'a [u8]) -> PyResult<PyObject> {
    if text.len() != 8 || key.len() != 16 {
        return Err(PyValueError::new_err("Wrong text or key size"));
    }
    let mut text = text.to_owned();
    tea::tea16_encrypt(text.as_mut_slice().try_into().unwrap(), key.try_into().unwrap());

    Ok(PyBytes::new(py, &text).into())
}

/// tea16_decrypt(text:bytes, key:bytes) -> bytes
/// --
///
/// Decrypt text with key using 16-rounds TEA
#[pyfunction]
fn tea16_decrypt<'a>(py: Python, text: &'a [u8], key: &'a [u8]) -> PyResult<PyObject> {
    if text.len() != 8 || key.len() != 16 {
        return Err(PyValueError::new_err("Wrong text or key size"));
    }
    let mut text = text.to_owned();
    tea::tea16_decrypt(text.as_mut_slice().try_into().unwrap(), key.try_into().unwrap());

    Ok(PyBytes::new(py, &text).into())
}

/// qqtea_encrypt(text:bytes, key:bytes) -> bytes
/// --
///
/// Encrypt text with key using 16-rounds QQ style TEA
#[pyfunction]
fn qqtea_encrypt<'a>(py: Python, text: &'a [u8], key: &'a [u8]) -> PyResult<PyObject> {
    if key.len() != 16 {
        return Err(PyValueError::new_err("Wrong key size"));
    }

    let text = block::qqtea_encrypt(text, key.try_into().unwrap());

    Ok(PyBytes::new(py, &text).into())
}

/// qqtea_decrypt(text:bytes, key:bytes) -> bytes
/// --
///
/// Decrypt text with key using 16-rounds QQ style TEA
#[pyfunction]
fn qqtea_decrypt<'a>(py: Python, text: &'a [u8], key: &'a [u8]) -> PyResult<PyObject> {
    if key.len() != 16 {
        return Err(PyValueError::new_err("Wrong key size"));
    }

    let text = block::qqtea_decrypt(text, key.try_into().unwrap());

    Ok(PyBytes::new(py, &text).into())
}

#[pymodule]
/// A Python module implemented in Rust.
fn rtea(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_debug, m)?)?;
    m.add_function(wrap_pyfunction!(tea16_encrypt, m)?)?;
    m.add_function(wrap_pyfunction!(tea16_decrypt, m)?)?;
    m.add_function(wrap_pyfunction!(qqtea_encrypt, m)?)?;
    m.add_function(wrap_pyfunction!(qqtea_decrypt, m)?)?;

    Ok(())
}
