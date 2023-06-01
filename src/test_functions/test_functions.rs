use pyo3::prelude::*;
use std::thread;
use std::mem;

use crate::matching::{get_matching_polies_stable_graph, get_deck, Graph, _calculate_matching_polynomial_binary, calculate_matching_polynomial_pointer};
use crate::matching_raw::{calculate_matching_polynomial_raw, GraphProperties, GraphData, get_deck as get_raw_deck};

//#[pyfunction]
//pub fn binary_test(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    //// set up the graph and the polies vector
    //let graph = data;
    //let graph_size = graph.graph_size();
    //let graph_clone_list = [graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone()];

    //let mut polies = Vec::<Vec<u64>>::new();

    //// now get that polynomial!
    //let graph_poly = _calculate_matching_polynomial_binary(graph);
    //polies.push(graph_poly[..=graph_size].to_vec());
    //println!("graph poly: {:?}", graph_poly[..=graph_size].to_vec());

    //for graph in graph_clone_list {
        //// spawn a thread for each graph in the deck
        //let poly = _calculate_matching_polynomial_binary(graph);
        //polies.push(poly[..=graph_size].to_vec());
    //}
    //Ok(polies)
//}

#[pyfunction]
pub fn raw_test(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    // set up the graph and the polies vector
    let graph = data;
    let graph_size = graph.graph_size();
    let graph_clone_list = [graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone()];

    let mut polies = Vec::<Vec<u64>>::new();

    // now get that polynomial!
    let graph_poly = calculate_matching_polynomial_raw(graph);
    polies.push(graph_poly[..=graph_size].to_vec());
    println!("graph poly: {:?}", graph_poly[..=graph_size].to_vec());

    for graph in graph_clone_list {
        // spawn a thread for each graph in the deck
        let poly = calculate_matching_polynomial_raw(graph);
        polies.push(poly[..=graph_size].to_vec());
    }
    Ok(polies)
}

#[pyfunction]
pub fn raw_multithreaded_test(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    // set up the graph and the polies vector
    let graph = data;
    let graph_size = graph.graph_size();
    let graph_clone_list = [graph.clone(), graph.clone(), graph.clone(), graph.clone()];
    let mut polies = Vec::<Vec<u64>>::new();

    let mut thread_handles = Vec::<thread::JoinHandle<[u64; mem::size_of::<usize>()*8]>>::new();

    let graph_poly = calculate_matching_polynomial_raw(graph);
    polies.push(graph_poly[..=graph_size].to_vec());
    println!("graph poly: {:?}", graph_poly[..=graph_size].to_vec());

    for graph in graph_clone_list {
        // spawn a thread for each graph in the deck
        let handle = thread::spawn(move || {
            calculate_matching_polynomial_raw(graph)
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        let poly = handle.join().unwrap();
        polies.push(poly[..=graph_size].to_vec());
    }
    Ok(polies)
}

/// This function tests whether or not the pointer_multithreaded function works
///  as wrutten, it appears to produce the same 
#[pyfunction]
pub fn pointer_multithreaded_test(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    // set up the graph and the polies vector
    let graph = Graph::from(data);
    let graph_size = graph.graph_size();
    let graph_clone_list = [graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone()];
    let mut polies = Vec::<Vec<u64>>::new();

    let mut thread_handles = Vec::<thread::JoinHandle<[u64; mem::size_of::<usize>()*8]>>::new();

    // calculate the poly for the graph
    let poly = calculate_matching_polynomial_pointer(graph);
    polies.push(poly[..=graph_size].to_vec());

    // calculate the poly for each element in the list of cloned graphs...
    for graph in graph_clone_list {
        // spawn a thread for each graph in the deck
        let handle = thread::spawn(move || {
            calculate_matching_polynomial_pointer(graph)
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        let poly = handle.join().unwrap();
        polies.push(poly[..=graph_size].to_vec());
    }
    Ok(polies)
}

#[pyfunction]
pub fn basic_multithreaded_test(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    // set up the graph and the polies vector
    let graph = Graph::from(data);
    let graph_size = graph.graph_size();
    let graph_clone_list = [graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone(), graph.clone()];
    let mut polies = Vec::<Vec<u64>>::new();

    let mut thread_handles = Vec::<thread::JoinHandle<[u64; mem::size_of::<usize>()*8]>>::new();

    let poly = calculate_matching_polynomial_pointer(graph);
    polies.push(poly[..=graph_size].to_vec());

    for graph in graph_clone_list {
        // spawn a thread for each graph in the deck
        let handle = thread::spawn(move || {
            calculate_matching_polynomial_pointer(graph)
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        let poly = handle.join().unwrap();
        polies.push(poly[..=graph_size].to_vec());
    }
    Ok(polies)
}
