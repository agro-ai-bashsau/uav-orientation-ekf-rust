
// Jetson Nano example (with ROS2)
use ekf_rs::Ekf;
use ros2_rust::subscribe_imu_data;

fn main() {
    let mut ekf = Ekf::new();

    subscribe_imu_data(|gyro, accel| {
        ekf.predict(gyro);
        ekf.update(accel);
        println!("Orientation quaternion: {:?}", ekf.x);
    });
}
