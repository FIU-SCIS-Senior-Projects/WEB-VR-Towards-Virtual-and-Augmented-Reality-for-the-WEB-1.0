#[macro_use]
extern crate serde_json;

mod spat_input;
mod devices;

use devices::mouse::{Event, MouseResolutions};

fn main() {

    let mut m = spat_input::InputAdapter::new_mouse();
    println!("Mouse output: {}",m.output);
    m.manipulation.move_mouse(400,200);
    m.update_output();
    println!("New Mouse output: {}",m.output);

}
