mod i2c;
mod sio;
mod smbus;
pub mod usb;

pub trait BusOp {
    fn write();

    fn read();
}