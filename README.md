# aahu-rs

Android Auto Head Unit - Rust

## Work In Progress

#### Goals

* Compare Rust tokio to C++ Boost::ASIO
* Determine a pattern for USB on Embedded Linux - not using Rust wrapped libusb


#### Current log output

     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([6, 48, 0, 0, 2, 0])
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([9, 33, 0, 1, 0, 1, 34, 77, 0])
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([6, 48, 0, 0, 2, 0])
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([9, 33, 17, 1, 0, 1, 34, 27, 0])
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([6, 48, 15, 0, 0, 0])
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([6, 48, 0, 0, 2, 0])
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([6, 48, 0, 0, 2, 0])
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([6, 48, 6, 0, 0, 0])
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([9, 33, 16, 1, 0, 1, 34, 27, 0])
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([8, 11, 0, 7, 1, 0, 32, 5])
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([6, 48, 0, 0, 2, 0])
     INFO  aahu                          > Bus 010 Device 014: ID 18d1:4ee1 - Google Pixel 4 XL
    Alignment 0 error invalid UTF-16 ignored string id 1
    Alignment 0 error invalid UTF-16 ignored string id 2
    Alignment 0 error invalid UTF-16 ignored string id 3
     DEBUG usbapi::os::linux::usb_device > Unknown descriptor type: Unknown([6, 48, 6, 0, 0, 0])
     DEBUG usbapi::os::linux::usbfs      > Not a usbfs device we need to unload kernel module if possible
     INFO  aahu                          > Protocol Version: 2
     INFO  aahu                          > device supports protocol 1 or higher
     
 Run with `RUST_LOG=Trace` to see output