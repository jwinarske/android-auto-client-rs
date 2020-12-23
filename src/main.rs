extern crate byteorder;
extern crate pretty_env_logger;

use std::sync::atomic::{AtomicBool};
use std::sync::Arc;
use usbapi::{UsbEnumerate, UsbCore, ControlTransfer, UsbDevice};
use usbapi::os::linux::usbfs::UsbFs;
use std::str;

use byteorder::{ByteOrder, LittleEndian};

#[macro_use]
extern crate log;

// Include the `items` module, which is generated from items.proto.
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/android.auto.proto.rs"));
}

const ACC_REQ_GET_PROTOCOL: u8 = 51;
const ACC_REQ_SEND_STRING: u8 = 52;
const ACC_REQ_START: u8 = 53;

const GOOGLE_VID: u16 = 0x18d1;
const ACC_PID: u16  = 0x2d00;

fn accessory_start(usb: &mut UsbFs) -> bool {
    match usb.control(ControlTransfer::new(
        0x40, ACC_REQ_START,0,0,None,100
    )) {
        Ok(_) => { return true; }
        Err(err) => warn!("Failed to start accessory mode {}", err),
    };

    return false;
}

fn send_string(usb: &mut UsbFs, index: u16, string: &'static str) -> bool {

    let buf: Vec<u8> = string.as_bytes().to_vec();
    match usb.control(ControlTransfer::new(
        0x40, ACC_REQ_SEND_STRING, 0, index, Option::from(buf), 100
    )) {
        Ok(_) => { return true; }
        Err(err) => warn!("Failed to start accessory mode {}", err),
    };

    return false;
}

fn get_protocol(usb: &mut UsbFs) -> u16 {

    let vec = vec![0 as u8; 2];
    match usb.control(ControlTransfer::new(
        0xC0,
        ACC_REQ_GET_PROTOCOL,
        0,
        0,
        Some(vec),
        100
    )) {
        Ok(data) => {
            let length = data.len();
            if length % 2 != 0 || length == 0 {
                warn!(
                    "Get Protocol length: {}", length
                );
                return 0;
            }

            let mut version = [0; 1];
            LittleEndian::read_u16_into(&data, &mut version);
            info!("Protocol Version: {:?}", version[0]);
            return version[0];
        }
        Err(err) => warn!("Failed to Get Protocol version {}", err),
    };

    return 0;
}

fn bus_info_print(device: &UsbDevice) {
    info!("Bus {:03} Device {:03}: ID {:04x}:{:04x} - {} {}",
          device.bus_num,
          device.dev_num,
          device.device.id_vendor,
          device.device.id_product,
          device.manufacturer,
          device.product);
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

fn enumerate_android_open_accessory() -> Result<(), Box<dyn Error>> {

    let usb = UsbEnumerate::from_sysfs()?;

    for (_bus_address, device) in usb.devices() {
        if device.device.id_vendor == GOOGLE_VID &&
            device.device.id_product == 0x4ee1 {
            bus_info_print(&device);
            let mut usb = UsbCore::from_device(&device).expect("Could not open device");

            let _ = usb.claim_interface(0).is_ok();

            let protocol = get_protocol(&mut usb);
            if protocol >= 1 {
                info!("device supports protocol 1 or higher");
            } else {
                return Err(Box::new(MyError("could not read device protocol version".into())));
            }

            send_string(&mut usb, 0, "Android");
            send_string(&mut usb, 1, "Android Auto");
            send_string(&mut usb, 2, "Android Auto");
            send_string(&mut usb, 3, "2.0.1");
            send_string(&mut usb, 4, "https://www.flashyconfidence.com");
            send_string(&mut usb, 5, "FC-AAAAAA001");

            if !accessory_start(&mut usb) {
                return Err(Box::new(MyError("failed to start".into())));
            }

        } else if device.device.id_vendor == GOOGLE_VID &&
            device.device.id_product == ACC_PID {
            bus_info_print(&device);
            info!("AOAv1 accessory");
        } else if device.device.id_vendor == GOOGLE_VID &&
            device.device.id_product == (ACC_PID + 1) {
            bus_info_print(&device);
            info!("AOAv1 accessory + adb");
        } else if device.device.id_vendor == GOOGLE_VID &&
            device.device.id_product == (ACC_PID + 2) {
            bus_info_print(&device);
            info!("AOAv2 audio");
        } else if device.device.id_vendor == GOOGLE_VID &&
            device.device.id_product == (ACC_PID + 3) {
            bus_info_print(&device);
            info!("AOAv2 audio + adb");
        } else if device.device.id_vendor == GOOGLE_VID &&
            device.device.id_product == (ACC_PID + 4) {
            bus_info_print(&device);
            info!("AOAv2 accessory + audio");
        } else if device.device.id_vendor == GOOGLE_VID &&
            device.device.id_product == (ACC_PID + 5) {
            bus_info_print(&device);
            info!("AOAv2 accessory + audio + adb");
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {

    pretty_env_logger::init();

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGQUIT, Arc::clone(&term)).unwrap();
    signal_hook::flag::register(signal_hook::SIGTERM, Arc::clone(&term)).unwrap();
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term)).unwrap();

    if let Err(e) = enumerate_android_open_accessory() {
        println!("{}", e);
    }
}
