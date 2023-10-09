#matching_poly_py03_binary_rep#:

Python bindings for `matching_poly_lib`; the library for calculating 
matching polynomials from graphs.

Installation:
First, you will need `matching_poly_lib`. Next, you will need `maturin`.
Activate your favourite python environment and run:

`pip install maturin`

Then, cd to the folder `matching_poly_py03_binary_rep`. and run:

`maturin develop --release`

This will install the relevant functions into your Python environment,
as package `matching-poly-binary` and available in Python scripts as 
as the module `matching_rs`.
