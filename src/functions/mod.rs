mod functions;
mod weighted_functions;

pub use functions::{
                   adaptive_matchings,
                   pointer_matchings,
                   pointer_matchings_multithreaded,
                   binary_matchings,
                   binary_matchings_multithreaded,
                   edge_matchings,
                   adjacency_matchings,
                   };

pub use weighted_functions::{
                   weighted_matchings,
                   weighted_matchings_parallel,
                   weighted_matchings_non_parallel,
                   weighted_matchings_addresses,
                   get_weighted_poly_from_addresses,
                   get_poly_addresses
};
