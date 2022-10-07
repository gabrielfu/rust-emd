use numpy::{PyReadonlyArray2};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

mod rust_fn;
use rust_fn::algorithms;

#[pymodule]
fn rugraphlib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn edmonds_karp<'py>(_py: Python<'_>, x: PyReadonlyArray2<f64>, s: i32, t: i32) -> f64 {
        let array = x.as_array();
        algorithms::edmonds_karp(&array, s, t)
    }

    Ok(())
}


