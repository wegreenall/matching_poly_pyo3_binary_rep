extern crate matching_poly_lib;
use matching_poly_lib::matching as matching;
use matching_poly_lib::matching_raw_memory as matching_raw;
use pyo3::prelude::*;
use std::mem;
mod test_functions;
use test_functions::{pointer_multithreaded_test, basic_multithreaded_test};

mod functions;
use functions::{calculate_matching_polynomial_pointer,
                calculate_matching_polynomial_pointer_multithreaded,
                calculate_matching_polynomial_from_binary_representation,
                calculate_matching_polynomial_from_binary_representation_multithreaded,
                calculate_matching_polynomial_from_edges,
                calculate_matching_polynomial_from_adjacency,
                raw_calculate_matching_polynomial,
                raw_calculate_matching_polynomial_multithreaded
               };


const MAX_NODES: usize = mem::size_of::<usize>()*8;

#[pymodule]
fn matching_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(calculate_matching_polynomial_from_edges))?;
    m.add_wrapped(wrap_pyfunction!(calculate_matching_polynomial_from_adjacency))?;
    m.add_wrapped(wrap_pyfunction!(calculate_matching_polynomial_from_binary_representation))?;
    m.add_wrapped(wrap_pyfunction!(calculate_matching_polynomial_from_binary_representation_multithreaded))?;
    m.add_wrapped(wrap_pyfunction!(calculate_matching_polynomial_pointer))?;
    m.add_wrapped(wrap_pyfunction!(calculate_matching_polynomial_pointer_multithreaded))?;
    m.add_wrapped(wrap_pyfunction!(raw_calculate_matching_polynomial))?;
    m.add_wrapped(wrap_pyfunction!(raw_calculate_matching_polynomial_multithreaded))?;

    // test functions
    m.add_wrapped(wrap_pyfunction!(basic_multithreaded_test))?;
    m.add_wrapped(wrap_pyfunction!(pointer_multithreaded_test))?;
    //m.add_wrapped(wrap_pyfunction!(raw_multithreaded_test))?;
    //m.add_wrapped(wrap_pyfunction!(raw_test))?;
    Ok(())
}
