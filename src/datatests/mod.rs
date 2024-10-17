//! Perform tests on real world data
//!
//! The data was accumulated listening to communication between
//! `rostopic pub` and `rostopic echo` for various standard messages.

mod nav_sat_fix;
mod pose;
mod pose_array;
mod pose_with_covariance;
mod sensor_msgs_image;
mod string;
