use serde::{Deserialize, Serialize};
use zpl_toolchain_print_client::{PrinterConfig, UsbPrinter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub name: String,
    pub uid: String,
    pub connection: Connect,
    pub provider: String,
    pub manufacturer: String,
    // Required in list but omitted in /write
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    // Required in /write but omitted in list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct Connect(u16, u16, String);

impl<'de> Deserialize<'de> for Connect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        let (vendorproduct, name) = string
            .strip_prefix("usb://")
            .unwrap()
            .split_once("/")
            .unwrap();

        let (vendor, product) = vendorproduct.split_once(":").unwrap();

        Ok(Connect(
            vendor.parse().unwrap(),
            product.parse().unwrap(),
            name.to_string(),
        ))
    }
}

impl Serialize for Connect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = format!("usb://{}:{}/{}", self.0, self.1, self.2);
        serializer.serialize_str(&value)
    }
}

impl Device {
    pub fn list() -> Vec<Self> {
        zpl_toolchain_print_client::UsbPrinter::list_devices()
            .into_iter()
            .filter(|dev| dev.0 == 0x0A5F)
            .map(|dev| Device {
                connection: Connect(dev.0, dev.1, dev.2.clone()),
                name: dev.2,
                device_type: Some("printer".to_string()),
                uid: "1234567890AB".to_string(),
                provider: "Not so Zebra Enterprise Connector".to_string(),
                manufacturer: "Zebra".to_string(),
                version: None,
            })
            .collect()
    }

    pub fn connect(&self) -> anyhow::Result<UsbPrinter> {
        let Connect(vendor_id, product_id, _name) = self.connection.clone();
        let config = PrinterConfig::default();
        Ok(UsbPrinter::find(vendor_id, product_id, config)?)
    }
}
