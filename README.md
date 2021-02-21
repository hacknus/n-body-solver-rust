# n_body_rust

A simple implementation of an n-body-solver written in rust by using direct force calculation. Leapfrog is used to propagate in time.

This program was developed on CLion on macOS.
Run it using command line arguments for path and number of steps
```
run --package n_body_rust --bin n_body_rust -- SolSystData.dat 1000
```