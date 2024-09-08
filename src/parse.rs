use serde::Deserialize;
use toml::Table;
use crate::controller::ControllerOp;

#[derive(Deserialize)]
pub struct DeviceList {
    pub USB: Vec<USB>,
    pub I2C: Vec<I2C>,
}

#[derive(Deserialize)]
pub struct USB {
    pub name: String,
    pub VID: u16,
    pub PID: u16,
}

#[derive(Deserialize)]
struct I2C {
    name: String,
    Address: u8,
}

pub fn parse_toml_list(path: &str) -> DeviceList {
    let doc = std::fs::read_to_string(path).unwrap();
    let devices_list: DeviceList = toml::from_str(doc.as_str()).unwrap();

    return devices_list;
}