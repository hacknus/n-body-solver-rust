# n-body-solver-rust

A simple implementation of an n-body-solver written in rust by using Barnes-Hut-Method (Tree). Leapfrog is used to propagate in time.

This program was developed on CLion on macOS and is not parallelized! To use the parallel version, switch to the "parallel" branch.
Run it using command line arguments for path and number of steps
```
run --package n_body_rust --bin n_body_rust -- solar_jfc.dat 1000
```
