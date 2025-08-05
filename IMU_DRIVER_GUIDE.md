
# IMU Driver Integration Guide

## MPU-9250

Implemented in `drivers/mpu9250.rs`. Provides:

- `read_gyro() -> [f64; 3]`
- `read_accel() -> [f64; 3]`

## Abstraction Layer

Create a trait `IMUDriver` with:

```rust
trait IMUDriver {
    fn read_gyro(&self) -> [f64; 3];
    fn read_accel(&self) -> [f64; 3];
}
```

This allows easy porting to other IMUs.
