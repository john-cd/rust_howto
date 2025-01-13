// // ANCHOR: example
// use openrr::JointTrajectoryClient;
// use openrr::RobotClient;
// use openrr::UrdfRobot;
// use std::sync::Arc;
// use tracing::info;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tracing_subscriber::fmt::init();

//     // Load the robot model from a URDF file.
//     let urdf_path = "path/to/your/robot.urdf"; // Replace with your URDF path
//     let urdf_robot = UrdfRobot::from_path(urdf_path)?;

//     // Create a RobotClient. This example uses a dummy client.
//     // In a real application, you would use a client that connects to your
// robot.     let robot_client =
// Arc::new(DummyRobotClient::new(urdf_robot.clone()));

//     // Create a JointTrajectoryClient.
//     let joint_trajectory_client =
// JointTrajectoryClient::new(robot_client.clone(), urdf_robot);

//     // Example trajectory
//     let joint_names = vec!["joint1".to_string(), "joint2".to_string()]; //
// Replace with your joint names     let positions = vec![0.5, 1.0];
//     let duration = std::time::Duration::from_secs(5);

//     info!("Sending trajectory");
//     joint_trajectory_client
//         .send_joint_positions(joint_names.clone(), positions, duration)
//         .await?;
//     info!("Trajectory sent");

//     tokio::time::sleep(duration).await;

//     // Example trajectory back to initial position
//     let positions = vec![0.0, 0.0];
//     let duration = std::time::Duration::from_secs(5);

//     info!("Sending trajectory back to initial position");
//     joint_trajectory_client
//         .send_joint_positions(joint_names, positions, duration)
//         .await?;
//     info!("Trajectory sent");

//     tokio::time::sleep(duration).await;

//     Ok(())
// }

// // Dummy Robot Client for testing
// struct DummyRobotClient {
//     urdf_robot: UrdfRobot,
// }

// impl DummyRobotClient {
//     pub fn new(urdf_robot: UrdfRobot) -> Self {
//         Self { urdf_robot }
//     }
// }

// #[async_trait::async_trait]
// impl openrr::RobotClient for DummyRobotClient {
//     async fn current_joint_positions(
//         &self,
//         joint_names: &[String],
//     ) -> Result<Vec<f64>, openrr::Error> {
//         // In a real implementation, you would get the current joint
// positions from the robot.         // This dummy implementation just returns
// zeros.         Ok(vec![0.0; joint_names.len()])
//     }

//     async fn send_joint_positions(
//         &self,
//         _joint_names: Vec<String>,
//         _positions: Vec<f64>,
//         _duration: std::time::Duration,
//     ) -> Result<(), openrr::Error> {
//         // In a real implementation, you would send the joint positions to
// the robot.         // This dummy implementation does nothing.
//         info!("DummyRobotClient: send_joint_positions called");
//         Ok(())
//     }

//     fn urdf_robot(&self) -> &UrdfRobot {
//         &self.urdf_robot
//     }
// }
// // ANCHOR_END: example

// #[test]
// #[ignore = "not yet implemented"]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/843)
