#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use futures_util::future::ready;
use futures_util::stream::StreamExt;
use tokio_udev::{EventType, MonitorBuilder};

use crate::android_open_accessory::AndroidAccessory;

mod android_open_accessory;

include!(concat!(env!("OUT_DIR"), "/android_auto_proto.rs"));

fn get_aoa_state(aoa: &AndroidAccessory, aoa_state: &mut bool) {
    match aoa.enumerate_android_open_accessory() {
        Ok(result) => {
            if result.ne(&aoa_state) {
                *aoa_state = result;
                println!("Android Accessory state = {}", aoa_state);
            }
        }
        Err(_) => {
            error!("Failed to enumerate Android Open Accessory");
        }
    };
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let aoa = AndroidAccessory::new();
    let mut aoa_state = false;
    get_aoa_state(&aoa, &mut aoa_state);

    let builder = MonitorBuilder::new()
        .expect("Couldn't create builder")
        .match_subsystem_devtype("usb", "usb_device")
        .expect("Failed to add filter for USB devices");

    let monitor = builder.listen().expect("Couldn't create MonitorSocket");
    monitor
        .for_each(|event| {
            if let Ok(event) = event {
                debug!(
                    "Hotplug event: {}: {}",
                    event.event_type(),
                    event.device().syspath().display()
                );

                match event.event_type() {
                    EventType::Add => get_aoa_state(&aoa, &mut aoa_state),
                    EventType::Unbind => get_aoa_state(&aoa, &mut aoa_state),
                    _ => {}
                };
            }
            ready(())
        })
        .await
}
