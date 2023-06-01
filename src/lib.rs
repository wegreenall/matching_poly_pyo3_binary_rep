extern crate matching_poly_lib;
use matching_poly_lib::matching as matching;
use matching_poly_lib::matching_raw_memory as matching_raw;
use pyo3::prelude::*;
//use crate::matching::{calculate_matching_polynomial_from_edges, calculate_matching_polynomial_from_adjacency, calculate_matching_polynomial_from_binary_representation, calculate_matching_polynomial_from_binary_representation_multithreaded};
use matching::{get_matching_polies_stable_graph, get_deck, Graph, _calculate_matching_polynomial_binary, _calculated_weighted_matching_polynomial_binary, calculate_matching_polynomial_pointer as calculate_matching_polynomial_pointer_rs};
use matching_raw::{calculate_matching_polynomial_raw, GraphProperties, GraphData, get_deck as get_raw_deck};

mod test_functions;
use test_functions::{raw_multithreaded_test, pointer_multithreaded_test, basic_multithreaded_test, raw_test};

use petgraph::{graph::{NodeIndex, UnGraph}, stable_graph::StableGraph};
use petgraph::Undirected;
use polynomial::Polynomial;
use std::mem;
use std::thread;

const MAX_NODES: usize = mem::size_of::<usize>()*8;

#[pyfunction]
pub fn raw_calculate_matching_polynomial(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    let graph = data;
    let graph_size = graph.graph_size();
    let deck = get_raw_deck(&graph);

    let mut polies = Vec::<Vec<u64>>::new();

    // now get that polynomial!
    let graph_poly = calculate_matching_polynomial_raw(graph);
    polies.push(graph_poly[..=graph_size].to_vec());

    for graph in deck {
        // spawn a thread for each graph in the deck
        let poly = calculate_matching_polynomial_raw(graph);
        polies.push(poly[..graph_size].to_vec());
    }
    Ok(polies)
}

#[pyfunction]
pub fn raw_calculate_matching_polynomial_multithreaded(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    let graph = data;
    let graph_size = graph.graph_size();
    let deck = get_raw_deck(&graph);

    let mut polies = Vec::<Vec<u64>>::new();

    let mut thread_handles = Vec::<thread::JoinHandle<[u64; mem::size_of::<usize>()*8]>>::new();

    let poly = calculate_matching_polynomial_raw(graph);
    polies.push(poly[..=graph_size].to_vec());

    for graph in deck {
        // spawn a thread for each graph in the deck
        let handle = thread::spawn(move || {
            calculate_matching_polynomial_raw(graph)
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        let poly = handle.join().unwrap();
        polies.push(poly[..graph_size].to_vec());
    }
    Ok(polies)
}

#[pyfunction] 
pub fn calculate_matching_polynomial_pointer(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    let graph = Graph::from(data);
    let graph_size = graph.graph_size();
    let deck = get_deck(&graph);
    let mut polies = Vec::<Vec<u64>>::new();

    // now get that polynomial!
    let graph_poly = calculate_matching_polynomial_pointer_rs(graph);
    polies.push(graph_poly[..=graph_size].to_vec());

    for graph in deck {
        // spawn a thread for each graph in the deck
        let poly = calculate_matching_polynomial_pointer_rs(graph);
        polies.push(poly[..graph_size].to_vec());
    }
    Ok(polies)
}

#[pyfunction] 
pub fn calculate_matching_polynomial_pointer_multithreaded(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    let graph = Graph::from(data);
    let graph_size = graph.graph_size();
    let deck = get_deck(&graph);

    let mut polies = Vec::<Vec<u64>>::new();

    let mut thread_handles = Vec::<thread::JoinHandle<[u64; mem::size_of::<usize>()*8]>>::new();

    let poly = calculate_matching_polynomial_pointer_rs(graph);
    polies.push(poly[..=graph_size].to_vec());

    for graph in deck {
        // spawn a thread for each graph in the deck
        let handle = thread::spawn(move || {
            calculate_matching_polynomial_pointer_rs(graph)
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        let poly = handle.join().unwrap();
        polies.push(poly[..graph_size].to_vec());
    }
    Ok(polies)
}


#[pyfunction]
pub fn calculate_matching_polynomial_from_binary_representation(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    let graph = Graph::from(data);
    let deck = get_deck(&graph);
    let mut polies = Vec::<Vec<u64>>::new();

    // now get that polynomial!
    let graph_poly = _calculate_matching_polynomial_binary(graph);
    polies.push(graph_poly.data().to_vec());

    for graph in deck {
        // spawn a thread for each graph in the deck
        let poly = _calculate_matching_polynomial_binary(graph);
        polies.push(poly.data().to_vec());
    }
    Ok(polies)
}



#[pyfunction]
pub fn calculate_matching_polynomial_from_binary_representation_multithreaded(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    // set up the graph and the polies vector
    let graph = Graph::from(data);
    let deck = get_deck(&graph);
    let mut polies = Vec::<Vec<u64>>::new();

    // set up the thread handles
    let mut thread_handles = Vec::<thread::JoinHandle<Polynomial<u64>>>::new();

    // get the base polynomial
    let graph_poly = _calculate_matching_polynomial_binary(graph);
    polies.push(graph_poly.data().to_vec());

    // get the deck polynomials
    for graph in deck {
        // spawn a thread for each graph in the deck
        let handle = thread::spawn(move || {

            _calculate_matching_polynomial_binary(graph)
        });
        thread_handles.push(handle);
        //let graph_poly = _calculate_matching_polynomial_binary(graph);
        //polies.push(graph_poly.data().to_vec());
    }

    // pull the data off the handles
    for handle in thread_handles {
        let graph_poly = handle.join().unwrap();
        polies.push(graph_poly.data().to_vec());
    }

    Ok(polies)

}

/// TODO: Implement the function for Weighted graphs!
pub fn calculate_weight_matching_polynomial_from_binary_representation(data: [usize; mem::size_of::<usize>()*8], weights: [f32; MAX_NODES * MAX_NODES]) -> Result<Vec<Vec<f32>>, std::io::Error> {
    let graph = Graph::from(data);
    let deck = get_deck(&graph);
    let mut polies = Vec::<Vec<f32>>::new();

    // now get that polynomial!
    let graph_poly = _calculate_weighted_matching_polynomial_binary(graph);
    polies.push(graph_poly.data().to_vec());

    for graph in deck {
        // spawn a thread for each graph in the deck
        let poly = _calculate_weighted_matching_polynomial_binary(graph);
        polies.push(poly.data().to_vec());
    }
    Ok(polies)
}

#[pyfunction]
pub fn calculate_matching_polynomial_from_adjacency(input_graph: Vec<Vec<i32>>) -> Result<Vec<Vec<u64>>, std::io::Error> {
    // Produce the graph
    // First, build the iterator of elements
    // If a node has no edges attached to it, construct a node, else construct the edges
    let node_count = input_graph.len();
    let mut graph = UnGraph::<i32, ()>::with_capacity(node_count, node_count);
    for (i, node) in input_graph.iter().enumerate(){
        // add the nodes and their edges to the graph
        // if the node has no edges, add it as a node
        if node.iter().sum::<i32>() == 0 {
            graph.add_node(i as i32);
        } else {
            // otherwise, add the edges
            for edge in node {
                graph.add_edge(NodeIndex::new(i), NodeIndex::new(*edge as usize), ());
            }
        }

    }
    let graph = StableGraph::<i32, (), Undirected>::from(graph);
    
    Ok(get_matching_polies_stable_graph(graph))
} 


#[pyfunction]
pub fn calculate_matching_polynomial_from_edges(graph: Vec<(u32, u32)>) -> Result<Vec<Vec<u64>>, std::io::Error> {
    // Produce the graph
    let graph = UnGraph::<i32, ()>::from_edges(&graph);
    let graph = StableGraph::<i32, (), Undirected>::from(graph);
    Ok(get_matching_polies_stable_graph(graph))
} 
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
    m.add_wrapped(wrap_pyfunction!(raw_multithreaded_test))?;
    m.add_wrapped(wrap_pyfunction!(raw_test))?;
    Ok(())
}
