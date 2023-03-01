use pyo3::prelude::*;
use crate::matching::{calculate_matching_polynomial_from_edges, calculate_matching_polynomial_from_adjacency, calculate_matching_polynomial_from_binary_representation};
mod matching;

//fn main() {
    ////let poly_1 = Polynomial::new(vec![1, 2, 3]);
    ////let poly_2 = Polynomial::new(vec![1, 2, 3]);

    ////println!("Poly: {}", (poly_1*poly_2).pretty("x"));
    ////let graph = linear_graph(10);
    ////let graph = fully_connected_graph(5);
    ////let graph = loop_graph(25);
    //let poly = calculate_matching_polynomial();
    ////println!("Poly: {}", poly.pretty("x"));
    //println!("Poly: {:?}", poly);
//}
 

#[pymodule]
fn matching_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(calculate_matching_polynomial_from_edges))?;
    m.add_wrapped(wrap_pyfunction!(calculate_matching_polynomial_from_adjacency))?;
    m.add_wrapped(wrap_pyfunction!(calculate_matching_polynomial_from_binary_representation))?;
    Ok(())
}
