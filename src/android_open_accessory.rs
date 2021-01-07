use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str;

use usbapi::{ControlTransfer, UsbCore, UsbDevice, UsbEnumerate};
use usbapi::os::linux::usbfs::UsbFs;

const ACC_REQ_GET_PROTOCOL: u8 = 51;
const ACC_REQ_SEND_STRING: u8 = 52;
const ACC_REQ_START: u8 = 53;

const GOOGLE_VID: u16 = 0x18d1;
const ACC_PID: u16 = 0x2d00;


#[derive(Debug)]
struct AoaError(String);

impl fmt::Display for AoaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for AoaError {}


#[derive(Debug)]
pub struct AndroidAccessory {}

impl AndroidAccessory {
    pub(crate) fn new() -> AndroidAccessory {
        return AndroidAccessory {};
    }
    fn accessory_start(&self, usb: &mut UsbFs) -> bool {
        match usb.control(ControlTransfer::new(
            0x40, ACC_REQ_START, 0, 0, None, 100,
        )) {
            Ok(_) => { return true; }
            Err(err) => warn!("Failed to start accessory mode {}", err),
        };

        return false;
    }

    fn send_string(&self, usb: &mut UsbFs, index: u16, string: &'static str) -> bool {
        let buf: Vec<u8> = string.as_bytes().to_vec();
        match usb.control(ControlTransfer::new(
            0x40, ACC_REQ_SEND_STRING, 0, index, Option::from(buf), 100,
        )) {
            Ok(_) => { return true; }
            Err(err) => warn!("Failed to start accessory mode {}", err),
        };

        return false;
    }

    fn get_protocol(&self, usb: &mut UsbFs) -> u16 {
        let vec = vec![0 as u8; 2];
        match usb.control(ControlTransfer::new(
            0xC0,
            ACC_REQ_GET_PROTOCOL,
            0,
            0,
            Some(vec),
            100,
        )) {
            Ok(data) => {
                let length = data.len();
                if length % 2 != 0 || length == 0 {
                    warn!(
                        "Get Protocol length: {}", length
                    );
                    return 0;
                }

                let version = unsafe {
                    std::mem::transmute::<[u8; 2], u16>(<[u8; 2]>::try_from(data).unwrap())
                };
                info!("Protocol Version: {:?}", version);
                return version;
            }
            Err(err) => warn!("Failed to Get Protocol version {}", err),
        };

        return 0;
    }

    fn bus_info_print(&self, device: &UsbDevice) {
        info!("Bus {:03} Device {:03}: ID {:04x}:{:04x} - {} {}",
              device.bus_num,
              device.dev_num,
              device.device.id_vendor,
              device.device.id_product,
              device.manufacturer,
              device.product);
    }

    pub fn enumerate_android_open_accessory(&self) -> Result<bool, Box<dyn Error>> {
        let mut result = false;
        let usb = UsbEnumerate::from_sysfs()?;

        for (_bus_address, device) in usb.devices() {
            if device.device.id_vendor == GOOGLE_VID &&
                device.device.id_product == 0x4ee1 {
                self.bus_info_print(&device);
                let mut usb = UsbCore::from_device(&device).expect("Could not open device");

                let _ = usb.claim_interface(0).is_ok();

                let protocol = self.get_protocol(&mut usb);
                if protocol >= 1 {
                    info!("device supports protocol 1 or higher");
                } else {
                    return Err(Box::new(AoaError("could not read device protocol version".into())));
                }

                self.send_string(&mut usb, 0, "Android");
                self.send_string(&mut usb, 1, "Android Auto");
                self.send_string(&mut usb, 2, "Android Auto");
                self.send_string(&mut usb, 3, "2.0.1");
                self.send_string(&mut usb, 4, "https://www.flashyconfidence.com");
                self.send_string(&mut usb, 5, "FC-AAAAAA001");

                if !self.accessory_start(&mut usb) {
                    return Err(Box::new(AoaError("failed to start".into())));
                }
            } else if device.device.id_vendor == GOOGLE_VID &&
                device.device.id_product == ACC_PID {
                self.bus_info_print(&device);
                info!("AOAv1 accessory");
                result = true;
            } else if device.device.id_vendor == GOOGLE_VID &&
                device.device.id_product == (ACC_PID + 1) {
                self.bus_info_print(&device);
                info!("AOAv1 accessory + adb");
                result = true;
            } else if device.device.id_vendor == GOOGLE_VID &&
                device.device.id_product == (ACC_PID + 2) {
                self.bus_info_print(&device);
                info!("AOAv2 audio");
                result = true;
            } else if device.device.id_vendor == GOOGLE_VID &&
                device.device.id_product == (ACC_PID + 3) {
                self.bus_info_print(&device);
                info!("AOAv2 audio + adb");
                result = true;
            } else if device.device.id_vendor == GOOGLE_VID &&
                device.device.id_product == (ACC_PID + 4) {
                self.bus_info_print(&device);
                info!("AOAv2 accessory + audio");
                result = true;
            } else if device.device.id_vendor == GOOGLE_VID &&
                device.device.id_product == (ACC_PID + 5) {
                self.bus_info_print(&device);
                info!("AOAv2 accessory + audio + adb");
                result = true;
            }
        }
        Ok(result)
    }
}