// publish internal modules for test/bench.
pub mod block;
pub mod tea;

use block::QQTea;
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    pyfunction, pymodule,
    types::{PyBytes, PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyResult, Python,
};

#[pyfunction]
fn is_debug() -> bool {
    cfg!(debug_assertions)
}

/// tea16_encrypt(text:bytes, key:bytes) -> bytes
/// --
///
/// Encrypt text with key using 16-rounds TEA
#[pyfunction]
fn tea16_encrypt<'a>(
    py: Python<'a>,
    data: &'a [u8],
    key: &'a [u8],
) -> PyResult<Bound<'a, PyBytes>> {
    let (data, key): (&[u8; 8], &[u8; 16]) = match (data.try_into(), key.try_into()) {
        (Ok(text), Ok(key)) => (text, key),
        _ => return Err(PyValueError::new_err("Wrong text or key size")),
    };

    let data = tea::tea16_encrypt(*data, *key);

    Ok(PyBytes::new(py, &data))
}

/// tea16_decrypt(text:bytes, key:bytes) -> bytes
/// --
///
/// Decrypt text with key using 16-rounds TEA
#[pyfunction]
fn tea16_decrypt<'a>(
    py: Python<'a>,
    data: &'a [u8],
    key: &'a [u8],
) -> PyResult<Bound<'a, PyBytes>> {
    let (data, key): (&[u8; 8], &[u8; 16]) = match (data.try_into(), key.try_into()) {
        (Ok(text), Ok(key)) => (text, key),
        _ => return Err(PyValueError::new_err("Wrong text or key size")),
    };

    let data = tea::tea16_decrypt(*data, *key);

    Ok(PyBytes::new(py, &data))
}

/// qqtea_encrypt(text:bytes, key:bytes) -> bytes
/// --
///
/// Encrypt text with key using 16-rounds QQ style TEA
#[pyfunction]
fn qqtea_encrypt<'a>(
    py: Python<'a>,
    data: &'a [u8],
    key: &'a [u8],
) -> PyResult<Bound<'a, PyBytes>> {
    let key: &[u8; 16] = key
        .try_into()
        .map_err(|_| PyValueError::new_err("Wrong key size"))?;

    let cipher = QQTea::new(*key);

    PyBytes::new_with(
        py,
        QQTea::estimate_ciphertext_size(data.len()),
        |buf| match cipher.encrypt_inout(data, buf) {
            Ok(_) => Ok(()),
            Err(_) => Err(PyRuntimeError::new_err(
                "insufficient output buffer, this is a internal error, and should not happen",
            )),
        },
    )
}

/// qqtea_decrypt(text:bytes, key:bytes) -> bytes
/// --
///
/// Decrypt text with key using 16-rounds QQ style TEA
#[pyfunction]
fn qqtea_decrypt<'a>(
    py: Python<'a>,
    data: &'a [u8],
    key: &'a [u8],
) -> PyResult<Bound<'a, PyBytes>> {
    let key: &[u8; 16] = key
        .try_into()
        .map_err(|_| PyValueError::new_err("Wrong key size"))?;

    let cipher = QQTea::new(*key);

    let mut plaintext_range = None;

    // use Python arena for faster allocation
    let temp_bytes = PyBytes::new_with(py, data.len(), |buf| {
        buf.copy_from_slice(data);

        plaintext_range = Some(
            cipher
                .decrypt_inout(buf)
                .map_err(|_| PyValueError::new_err("Bad ciphertext"))?,
        );

        Ok(())
    })?;

    Ok(PyBytes::new(py, &temp_bytes[plaintext_range.unwrap()]))
}

#[pymodule]
/// A Python module implemented in Rust.
fn rtea(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_debug, m)?)?;
    m.add_function(wrap_pyfunction!(tea16_encrypt, m)?)?;
    m.add_function(wrap_pyfunction!(tea16_decrypt, m)?)?;
    m.add_function(wrap_pyfunction!(qqtea_encrypt, m)?)?;
    m.add_function(wrap_pyfunction!(qqtea_decrypt, m)?)?;

    Ok(())
}
