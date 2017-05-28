pub mod mouse;

use spat_input::InputAdapter;
use devices::mouse::{Event, MouseResolutions};

pub fn handle_mouse(dev: &mut InputAdapter<mouse::Manipulation,mouse::Input,String,mouse::State>){

    println!("Mouse output: {}",dev.output);
    dev.manipulation.move_mouse(400,200);
    dev.update_output();
    println!("New Mouse output: {}",dev.output);
    dev.manipulation.move_mouse(1,555);
    dev.update_output();
    println!("New Mouse output: {}",dev.output);
}
