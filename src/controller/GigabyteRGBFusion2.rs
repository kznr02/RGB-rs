use crate::interface::usb::USBDevice;

use super::ControllerOp;

#[derive(Debug, Clone)]
struct GigabyteRGBFusion2 {
    bus: USBDevice,
}

impl ControllerOp for GigabyteRGBFusion2 {

}