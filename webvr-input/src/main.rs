#[macro_use]
extern crate serde_json;
extern crate libusb;
extern crate sdl2;

mod spat_input;
mod devices;

use std::thread::spawn; //For threads
use spat_input::InputAdapter;

use sdl2::event::{Event};
use std::collections::HashSet;
use std::time::Duration;

fn main() {
    // start sdl2 with everything
    let context = sdl2::init().unwrap();
    let video_context = context.video().unwrap();

    // Create a window and show it| Eventually this will be servo's job
    let mut window  = match video_context.window("Web-vr input", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    {
        window.show();

        //Makes sure that the mouse cursor is shown in the window
        context.mouse().show_cursor(true);
    }

    let mut events = context.event_pump().unwrap();
    let mut m = InputAdapter::new_mouse();

    // loop until we receive a QuitEvent
    'event : loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _               => continue
            }
        }

        // get a mouse state
        let state = events.mouse_state();

        devices::handle_mouse(&mut m, &state);

        std::thread::sleep(Duration::from_millis(100));
    }
}
