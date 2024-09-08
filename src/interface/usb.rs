use rusb::{self, DeviceHandle, DeviceList, GlobalContext};
use crate::parse;
#[derive(Debug, Clone)]
pub struct USBDevice {
    pub name: String,
    pub vendor_id: u16,
    pub product_id: u16,
}

impl super::BusOp for USBDevice {
    fn read() {
        
    }

    fn write() {
        
    }
}

pub fn get_all_usb_devices() -> DeviceList<GlobalContext> {
    let devices = rusb::devices().unwrap();

    return devices;
}

pub fn get_exist_usb_devices(list: &parse::DeviceList) -> Vec<DeviceHandle<GlobalContext>> {
    let mut devices:Vec<DeviceHandle<GlobalContext>> = Vec::new();
    for item in &list.USB {
        if let Some(device) = rusb::open_device_with_vid_pid(item.VID, item.PID) {
            println!("Device name[{}] Pid[0x{:04x}] Vid[0x{:04x}] detected!", item.name, item.PID, item.VID);
            devices.push(device);
        } else {
            continue;
        }
    }

    return devices;
}