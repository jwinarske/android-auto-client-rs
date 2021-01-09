#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use futures_util::future::ready;
use futures_util::stream::StreamExt;
use tokio_udev::{EventType, MonitorBuilder};

use crate::android_open_accessory::AndroidAccessory;
use crate::aap::TransportType;

include!(concat!(env!("OUT_DIR"), "/android_auto_proto.rs"));

mod android_open_accessory;
mod window;
mod aap;


fn get_aoa_state(aoa: &AndroidAccessory, aoa_state: &mut bool) -> bool {
    match aoa.enumerate_android_open_accessory() {
        Ok(result) => {
            if result.ne(&aoa_state) {
                *aoa_state = result;
                println!("Android Accessory state = {}", aoa_state);
                return result;
            }
        }
        Err(_) => {
            error!("Failed to enumerate Android Open Accessory");
        }
    };
    return false;
}

//
// Main thread is responsible for monitoring USB events
// and starting and stopping the Android Auto stack
//

#[tokio::main]
async fn main() {

    // get program options here...

    pretty_env_logger::init();

    let mut aap = aap::Server::new();
    let transport = TransportType::USB;
    let ip_address: &str = "192.168.1.120";

    let aoa = AndroidAccessory::new();
    let mut aoa_state = false;
    let mut prior_state = false;
    if get_aoa_state(&aoa, &mut aoa_state) {
        aap.start(&transport, &ip_address);
    };

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
                    EventType::Add | EventType::Unbind => {
                        if get_aoa_state(&aoa, &mut aoa_state) {
                            prior_state = aoa_state;
                            match aap.state() {
                                aap::State::Initial | aap::State::Stopped => {
                                    aap.start(&transport, &ip_address);
                                },
                                _ => {}
                            }
                        }
                        else if prior_state {
                            prior_state = false;
                            aap.shutdown();
                        }
                    },
                    _ => {}
                };
            }
            ready(())
        })
        .await
}
