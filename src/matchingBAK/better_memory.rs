#[pyfunction]
pub fn calculate_matching_polynomial_from_binary_representation_multithreaded(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    let graph = Graph::new(data);
    let deck = get_deck(&graph);
    let mut polies = Vec::<Vec<u64>>::new();

    let mut thread_handles = Vec::<thread::JoinHandle<Polynomial<u64>>>::new();

    // now get that polynomial!
    let graph_poly = _calculate_matching_polynomial_binary(graph);
    polies.push(graph_poly.data().to_vec());

    for graph in deck {
        // spawn a thread for each graph in the deck
        let handle = thread::spawn(move || {
            _calculate_matching_polynomial_binary(graph)
        });
        thread_handles.push(handle);
        //let graph_poly = _calculate_matching_polynomial_binary(graph);
        //polies.push(graph_poly.data().to_vec());
    }
    for handle in thread_handles {
        let graph_poly = handle.join().unwrap();
        polies.push(graph_poly.data().to_vec());
    }

    Ok(polies)

}

#[pyfunction]
pub fn calculate_matching_polynomial_from_binary_representation(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    let graph = Graph::new(data);
    let deck = get_deck(&graph);
    let mut polies = Vec::<Vec<u64>>::new();

    // We want to add a preallocated memory set. The capacity required is large
    // but we need to pre-calculate it. Each Graph on a 64 bit machine
    // costs 64 64-bit words, or 512 bytes. 
    // The allocation for a full 64 node graph would be far too massive.

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

pub fn _calculate_matching_polynomial_binary(graph: Graph, depth: usize, data: &mut Vec<Graph>) -> Polynomial<u64> {
    // the base case for the process is that the graph is edgeless.
    // This means that, of the remaining nodes, each of their integer
    // representations is a power of two.
    if graph.edgeless() { // i.e. we're at the base case.
        // produce a sequence of coefficients the same length as the number of vertices
        //println!("Hit edgeless graph! with {} nodes", graph.edgeless_node_count());
        let mut coeffics = vec![0; graph.edgeless_node_count()];
        coeffics.push(1);
        let poly = Polynomial::new(coeffics);
        //println!("Polynomial: {:?}", poly);
        //println!("graph {:?}", graph.data);
        return poly
    } else {
        // get G' and G''
        // G' = G - an edge
        // G'' = G - the nodes connected to the edge removed to get G'
        //println!("graph {:?}", &graph.data);
        let (graph_prime, graph_prime_prime) = graph.get_graph_primes();
        //println!("graph {:?}", &graph.data);
        //println!("graph_prime {:?}", &graph_prime.data);
        //println!("graph_prime_prime {:?}", &graph_prime_prime.data);

        let poly_1 = _calculate_matching_polynomial_binary(graph_prime);
        let poly_2 = _calculate_matching_polynomial_binary(graph_prime_prime);
        let poly = poly_1 + poly_2;
        return poly
    }
} 
