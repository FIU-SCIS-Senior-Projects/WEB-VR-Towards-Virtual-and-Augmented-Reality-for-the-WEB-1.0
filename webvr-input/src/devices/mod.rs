extern crate libusb;
extern crate sdl2;

pub mod mouse;
pub mod empty_device; //added for empty device library

use spat_input::InputAdapter;
use devices::mouse::{Event, MouseResolutions};
use std::collections::HashSet;
use std::time::Duration;

use sdl2::mouse::MouseState;
use std::thread; //For threads


pub fn handle_mouse(dev: &mut InputAdapter<ConnectionMode, mouse::Input,
                    mouse::State, String, mouse::Resolution, mouse::Event>,
                    state: &MouseState){

    let mut prev_buttons = HashSet::new();

    // Create a set of pressed Keys.
    let buttons = state.pressed_mouse_buttons().collect();

    // Get the difference between the new and old sets.
    let new_buttons = &buttons - &prev_buttons;
    let old_buttons = &prev_buttons - &buttons;

    if !new_buttons.is_empty() || !old_buttons.is_empty() {

        dev.move_mouse(state.x(), state.y());
        let button_down = format!("{:?}", new_buttons);

        dev.state = match button_down.as_ref(){
            "{Left}" => mouse::State::LeftButtonDown,
            "{Right}" => mouse::State::RightButtonDown,
            "{Middle}" => mouse::State::MiddleButtonDown,
            "{Left, Right}" | "{Right, Left}"   => mouse::State::LeftRight,
            "{Left, Middle}"  | "{Middle, Left}"  => mouse::State::LeftMiddle,
            "{Right, Middle}"  | "{Middle, Right}"=> mouse::State::RightMiddle,
            _=> mouse::State::Idle
        };

        dev.update_output();
        println!("New Mouse output: {}",dev.output);
    }

}
