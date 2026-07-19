use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use light_poseidon::{Poseidon, PoseidonBytesHasher, PoseidonHasher as _};
use pyo3::prelude::*;

#[pyclass]
struct Hasher {
    inner: Poseidon<Fr>,
}

#[pymethods]
impl Hasher {
    #[new]
    fn new(nr_inputs: usize) -> PyResult<Self> {
        let inner = Poseidon::<Fr>::new_circom(nr_inputs)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;
        Ok(Self { inner })
    }

    fn hash(&mut self, inputs: Vec<u64>) -> PyResult<String> {
        let fr_inputs: Vec<Fr> = inputs.into_iter().map(Fr::from).collect();
        let result = self
            .inner
            .hash(&fr_inputs)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;
        Ok(format!("0x{}", hex::encode(result.into_bigint().to_bytes_be())))
    }

    fn hash_raw(&mut self, inputs: Vec<u64>) -> PyResult<Vec<u8>> {
        let fr_inputs: Vec<Fr> = inputs.into_iter().map(Fr::from).collect();
        let result = self
            .inner
            .hash(&fr_inputs)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;
        Ok(result.into_bigint().to_bytes_be().to_vec())
    }

    fn hash_bytes_be(&mut self, inputs: Vec<Vec<u8>>) -> PyResult<String> {
        let byte_refs: Vec<&[u8]> = inputs.iter().map(|v| v.as_slice()).collect();
        let result = self
            .inner
            .hash_bytes_be(&byte_refs)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;
        Ok(format!("0x{}", hex::encode(result)))
    }

    fn hash_bytes_le(&mut self, inputs: Vec<Vec<u8>>) -> PyResult<String> {
        let byte_refs: Vec<&[u8]> = inputs.iter().map(|v| v.as_slice()).collect();
        let result = self
            .inner
            .hash_bytes_le(&byte_refs)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;
        Ok(format!("0x{}", hex::encode(result)))
    }
}

#[pyfunction]
fn poseidon_hash(inputs: Vec<u64>) -> PyResult<String> {
    let mut hasher = Poseidon::<Fr>::new_circom(inputs.len())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;
    let fr_inputs: Vec<Fr> = inputs.into_iter().map(Fr::from).collect();
    let result = hasher
        .hash(&fr_inputs)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;
    Ok(format!("0x{}", hex::encode(result.into_bigint().to_bytes_be())))
}

#[pyfunction]
fn poseidon_hash_bytes(inputs: Vec<Vec<u8>>) -> PyResult<String> {
    let mut hasher = Poseidon::<Fr>::new_circom(inputs.len())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;
    let byte_refs: Vec<&[u8]> = inputs.iter().map(|v| v.as_slice()).collect();
    let result = hasher
        .hash_bytes_be(&byte_refs)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;
    Ok(format!("0x{}", hex::encode(result)))
}

#[pymodule]
fn light_poseidon_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(poseidon_hash, m)?)?;
    m.add_function(wrap_pyfunction!(poseidon_hash_bytes, m)?)?;
    m.add_class::<Hasher>()?;
    m.add("__version__", "0.1.0")?;
    Ok(())
}