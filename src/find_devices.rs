use std::io::Error;

use cmd_lib::*;

use crate::Device;

pub fn find_available_devices() -> Result<Vec<Device>, Error> {
    let printers = run_fun!(lpstat r"-v")?;
    let devices = printers
        .lines()
        .filter(|c| c.contains("Zebra%20Technologies"))
        .map(|line| {
            line.trim_start_matches("device for ")
                .split(": ")
                .collect::<Vec<&str>>()
        })
        .map(|name| Device {
            name: name[0].to_string(),
            device_type: "printer".to_string(),
            connection: name[1].to_string(),
            uid: "1234567890AB".to_string(),
            provider: "Not so Zebra Enterprise Connector".to_string(),
            manufacturer: "Zebra".to_string(),
        });

    Ok(devices.collect())
}
