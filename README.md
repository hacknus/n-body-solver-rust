# n-body-solver-rust

A simple implementation of an n-body-solver written in rust by using direct force calculation. Leapfrog is used to propagate in time.

This program was developed on CLion on macOS and uses openMPI (rsmpi) to parallelise the calculation
Run it using command line arguments for path and number of steps
```
mpirun -n 4 --package n_body_rust --bin n_body_rust -- solar_jfc.dat 1000
```
