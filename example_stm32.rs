
// STM32F4 usage example
// Assumes MPU-9250 is connected via I2C

fn main() {
    let imu = Mpu9250::new(i2c);
    let mut ekf = Ekf::new();

    loop {
        let gyro = imu.read_gyro();
        let accel = imu.read_accel();
        ekf.predict(gyro);
        ekf.update(accel);
    }
}
