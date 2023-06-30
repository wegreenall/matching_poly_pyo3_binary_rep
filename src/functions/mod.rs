mod functions;

pub use functions::{
                   raw_calculate_matching_polynomial,
                   raw_calculate_matching_polynomial_multithreaded,
                   calculate_matching_polynomial_pointer,
                   calculate_matching_polynomial_pointer_multithreaded,
                   calculate_matching_polynomial_from_binary_representation,
                   calculate_matching_polynomial_from_binary_representation_multithreaded,
                   calculate_matching_polynomial_from_edges,
                   calculate_matching_polynomial_from_adjacency,
                   };

