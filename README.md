
# Rust EKF for Embedded Systems

This repository contains the source code for a quaternion-based Extended Kalman Filter (EKF) implemented in Rust with `no_std` support for real-time orientation estimation on embedded platforms.

## Features

- Quaternion-based EKF formulation
- 4th-order Runge–Kutta integration
- Real-time performance at 1kHz
- Memory-safe and zero dynamic allocation (no heap)
- IMU abstraction layer
- Support for STM32F4 and Jetson Nano

## Repository Structure

- `src/`: Rust source code
- `examples/`: Usage examples for supported platforms
- `docs/`: API documentation (generated via rustdoc)
- `drivers/`: MPU-9250 and IMU abstraction

## Supported Platforms

- STM32F4 (with `cortex-m-rt`)
- Jetson Nano (Linux with `std`)

## Getting Started

```bash
cargo build --release --target thumbv7em-none-eabihf
```

## License

MIT License
