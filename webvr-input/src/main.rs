#[macro_use]
extern crate serde_json;

mod spat_input;
mod devices;

fn main() {

    let m = spat_input::InputAdapter::new_mouse();
    println!("Mouse output: {}",m.output);

}
