mod find_devices;
mod print;

pub use find_devices::find_available_devices;
pub use print::print_label;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Device {
    pub name: String,
    pub device_type: String,
    pub connection: String,
    pub uid: String,
    pub provider: String,
    pub manufacturer: String,
}
