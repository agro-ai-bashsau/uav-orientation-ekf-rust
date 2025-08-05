
# API Documentation

This crate provides the following public modules:

## `ekf::Ekf`

Main filter structure. Contains:

- `x`: State vector [q0, q1, q2, q3, bx, by, bz]
- `P`: Covariance matrix
- `Q`: Process noise matrix
- `R`: Measurement noise matrix

## `ekf::predict()`

Performs the prediction step using gyroscope data.

## `ekf::update()`

Performs the update step using accelerometer measurements.
