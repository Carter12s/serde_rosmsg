//! Perform test with sensor_msgs/NavSatFix

#[cfg(test)]
mod tests {
    use from_slice;

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct NavSatFix {
        pub r#header: self::Header,
        pub r#status: self::NavSatStatus,
        pub r#latitude: f64,
        pub r#longitude: f64,
        pub r#altitude: f64,
        pub r#position_covariance: ::std::vec::Vec<f64>,
        pub r#position_covariance_type: u8,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct NavSatStatus {
        pub r#status: i8,
        pub r#service: u16,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct Header {
        pub r#seq: u32,
        pub r#stamp: Time,
        pub r#frame_id: ::std::string::String,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct Time {
        // Note: rosbridge appears to accept secs and nsecs in for time without issue?
        // Not sure we should actually rely on this behavior, but ok for now...

        // This alias is required for ros2 where field has been renamed
        #[serde(alias = "sec")]
        pub secs: u32,
        // This alias is required for ros2 where field has been renamed
        #[serde(alias = "nanosec")]
        pub nsecs: u32,
    }

    #[test]
    fn read_nav_sat_fix() {
        let msg = from_slice::<NavSatFix>(include_bytes!("nav_sat_fix.bin")).unwrap();
    }
}
