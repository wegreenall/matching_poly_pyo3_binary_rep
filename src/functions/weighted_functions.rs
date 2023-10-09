extern crate matching_poly_lib;
use matching_poly_lib::weighted_graph_matching as weighted_matching;
use matching_poly_lib::weighted_graph_matching::{weighted_coefficient_calculation, weighted_matching_polynomial_addresses};
use weighted_matching::{WeightedGraph,_calculate_weighted_matching_polynomial_binary, get_weighted_deck,  weighted_polynomial_calculation, weighted_matching_polynomial_from_addresses}; 
use matching_poly_lib::binary_graph_matching::{BinaryGraph, calculate_matching_polynomial_pointer_addresses};
use pyo3::prelude::*;
use std::mem;
use std::thread;

const MAX_NODES: usize = mem::size_of::<usize>()*8;
const POLY_SIZE: usize = mem::size_of::<usize>()*8;

/// TODO: Implement the function for Weighted graphs!
#[pyfunction]
pub fn weighted_matchings(data: [usize; mem::size_of::<usize>()*8], weights: [f32; MAX_NODES * MAX_NODES]) -> Result<Vec<f32>, std::io::Error> {
    let weighted_graph = WeightedGraph::from(data, weights);

    // now get that polynomial!
    let graph_poly = _calculate_weighted_matching_polynomial_binary(weighted_graph);

    Ok(graph_poly.data().to_vec())
}

#[pyfunction]
pub fn weighted_matchings_parallel(weights: [f32; MAX_NODES * MAX_NODES], graph_size: usize)-> Result<Vec<f32>, std::io::Error> {
    let mut poly = [0.0; POLY_SIZE]; //Vec::<f32>::new();
    let mut thread_handles = Vec::<thread::JoinHandle<(usize, f32)>>::new();

    for coeffic in 0..=graph_size {
        let handle = thread::spawn(move || {
            (coeffic, weighted_coefficient_calculation(&weights, graph_size, coeffic))
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        let (coeffic, coeffic_val) = handle.join().unwrap();
        poly[coeffic] = coeffic_val; 
    }
    let poly = poly.to_vec();
    Ok(poly)
}

#[pyfunction]
pub fn weighted_matchings_non_parallel(weights: [f32; MAX_NODES * MAX_NODES], graph_size: usize)-> Result<Vec<f32>, std::io::Error> {
    Ok(weighted_polynomial_calculation(&weights, graph_size).to_vec())
}

#[pyfunction]
pub fn weighted_matchings_addresses(data:[usize; MAX_NODES], weights:[f32; MAX_NODES * MAX_NODES])-> Result<Vec<f32>, std::io::Error> {
    let graph = BinaryGraph::from(data);    
    let poly = weighted_matching_polynomial_addresses(graph, &weights);
    Ok(poly.to_vec())
}

#[pyfunction]
pub fn get_poly_addresses(data: [usize; MAX_NODES])-> Result<Vec<usize>, std::io::Error> {
    let graph = BinaryGraph::from(data);
    let (_, addresses) = calculate_matching_polynomial_pointer_addresses(graph);
    Ok(addresses)
}

#[pyfunction]
pub fn get_weighted_poly_from_addresses(addresses: Vec<usize>, weights: [f32; MAX_NODES * MAX_NODES], graph_size: usize) -> Result<Vec<f32>, std::io::Error> {
    let poly = weighted_matching_polynomial_from_addresses(addresses, &weights, graph_size);
    Ok(poly.to_vec())
}
