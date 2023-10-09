extern crate matching_poly_lib;
use matching_poly_lib::graph_matching as matching;
use matching_poly_lib::weighted_graph_matching as weighted_matching;
use pyo3::prelude::*;
use std::mem;
mod test_functions;
use test_functions::{pointer_multithreaded_test, basic_multithreaded_test};

mod functions;
use functions::{adaptive_matchings,
                pointer_matchings,
                pointer_matchings_multithreaded,
                binary_matchings,
                binary_matchings_multithreaded,
                edge_matchings,
                adjacency_matchings,
                weighted_matchings,
                weighted_matchings_parallel,
                weighted_matchings_non_parallel,
                get_weighted_poly_from_addresses,
                get_poly_addresses,
                weighted_matchings_addresses
               };

#[pymodule]
fn matching_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(adaptive_matchings))?;
    m.add_wrapped(wrap_pyfunction!(edge_matchings))?;
    m.add_wrapped(wrap_pyfunction!(adjacency_matchings))?;
    m.add_wrapped(wrap_pyfunction!(binary_matchings))?;
    m.add_wrapped(wrap_pyfunction!(binary_matchings_multithreaded))?;
    m.add_wrapped(wrap_pyfunction!(pointer_matchings))?;
    m.add_wrapped(wrap_pyfunction!(pointer_matchings_multithreaded))?;
    m.add_wrapped(wrap_pyfunction!(basic_multithreaded_test))?;
    m.add_wrapped(wrap_pyfunction!(pointer_multithreaded_test))?;

    m.add_wrapped(wrap_pyfunction!(weighted_matchings))?;
    m.add_wrapped(wrap_pyfunction!(weighted_matchings_parallel))?;
    m.add_wrapped(wrap_pyfunction!(weighted_matchings_non_parallel))?;
    m.add_wrapped(wrap_pyfunction!(weighted_matchings_addresses))?;
    m.add_wrapped(wrap_pyfunction!(get_weighted_poly_from_addresses))?;

    m.add_wrapped(wrap_pyfunction!(get_poly_addresses))?;
    Ok(())
}
