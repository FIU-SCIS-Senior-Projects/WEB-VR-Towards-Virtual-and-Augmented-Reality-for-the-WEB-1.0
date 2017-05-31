#[macro_use]
extern crate serde_json;
extern crate libusb;
extern crate os_type;

mod spat_input;
mod devices;

use std::thread::spawn; //For threads
use spat_input::InputAdapter;

fn main() {

    let mut m;
    //Starting off by using the libusb library to read usb devices
    let context = libusb::Context::new().unwrap();

    let mut os = os_type::current_platform();

    print!("Current Operating system: {:?}\n", os.os_type);
    print!("version: {}\n", os.version);

    match os.os_type {
        os_type::OSType::Ubuntu => {
            println!("It's Ubuntu");
        },
        _ => {}
    }

    for device in context.devices().unwrap().iter() {
        //TODO: Figure out a way to match product_id to different products
        let device_desc = device.device_descriptor().unwrap();

        //Pattern matching to product_id to figure out what type of decice
        //that We're dealing with. Ideally, we'll have a list of supported
        //device that we can use. For now, it's focused on a set of 
        //predetermined evices
        
        match device_desc.product_id() {
            49242 => {
                println!("It's the mouse");
                m = InputAdapter::new_mouse();

                //Spawning off a thread to handle the mouse even
                //TODO: THis might need reractoring later based on device type
                spawn(move||{
                    devices::handle_mouse(&mut m);
                });
            },
            _=> {}
        }
    }

}

