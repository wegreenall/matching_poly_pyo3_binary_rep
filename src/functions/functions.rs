extern crate matching_poly_lib;
use matching_poly_lib::graph_matching as matching;
use matching_poly_lib::weighted_graph_matching as weighted_matching;
use matching_poly_lib::matching_raw_memory as matching_raw;
use matching_poly_lib::petgraph as petgraph_matching;
use pyo3::prelude::*;
use matching::{get_deck, Graph, _calculate_matching_polynomial_binary,  calculate_matching_polynomial_pointer as calculate_matching_polynomial_pointer_rs};
use weighted_matching::{WeightedGraph,_calculate_weighted_matching_polynomial_binary, get_weighted_deck}; 
use matching_raw::{calculate_matching_polynomial_raw, GraphProperties, get_deck as get_raw_deck};
use petgraph_matching::{get_matching_polies_stable_graph};

use petgraph::{Undirected, stable_graph::StableGraph};
use petgraph::graph::{UnGraph, NodeIndex};
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
#[pyfunction]
pub fn calculate_weighted_matching_polynomial(data: [usize; mem::size_of::<usize>()*8], weights: [f32; MAX_NODES * MAX_NODES]) -> Result<Vec<Vec<f32>>, std::io::Error> {
    let weighted_graph = WeightedGraph::from(data, weights);
    let deck = get_weighted_deck(&weighted_graph);
    let mut polies = Vec::<Vec<f32>>::new();

    // now get that polynomial!
    let graph_poly = _calculate_weighted_matching_polynomial_binary(weighted_graph);
    polies.push(graph_poly.data().to_vec());

    for weighted_graph in deck {
        // spawn a thread for each graph in the deck
        let poly = _calculate_weighted_matching_polynomial_binary(weighted_graph);
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
