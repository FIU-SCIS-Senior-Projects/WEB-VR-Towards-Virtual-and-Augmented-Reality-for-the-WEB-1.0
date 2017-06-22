#[macro_use]
extern crate serde_json;
extern crate libusb;
extern crate sdl2;


mod spat_input;
mod devices;

use std::thread::spawn; //For threads
use spat_input::{InputAdapter, SubInput};

use sdl2::event::{Event};
use sdl2::controller::{Button, Axis };
use std::collections::HashSet;
use std::time::Duration;
use devices::{mouse, controller};

fn main() {
    // start sdl2 with everything
    let context = sdl2::init().unwrap();
    let video_context = context.video().unwrap();
    let controller_context= context.game_controller().unwrap();

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
    let available_controllers =
        match controller_context.num_joysticks() {
            Ok(n)  => n,
            Err(e) => panic!("can't enumerate joysticks: {}", e),
        };
    let mut controller = None;

    // Iterate over all available joysticks and look for game
    // controllers.
    for id in 0..available_controllers {
        if controller_context.is_game_controller(id) {
            println!("Attempting to open controller {}", id);

            match controller_context.open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    println!("Success: opened \"{}\"", c.name());
                    controller = Some(c);
                    break;
                },
                Err(e) => println!("failed: {:?}", e),
            }

        } else {
             println!("{} is not a game controller", id);
        }
    }

    let controller =
        match controller {
            Some(c) => c,
            None     => panic!("Couldn't open any controller"),
        };

    println!("Controller mapping: {}", controller.mapping());

    let mut inputs = controller::Input::new();
    let mut resolutions = controller::Resolution::new();

    //Simulating adding inputs as a developer. The developer would first need
    //to find out which inputs are available
    
    
    //creates a subinput for the y-axis
    let left_y_axis = SubInput::new_int(-32768, 32768);
    let a_button = "A".to_string();
    inputs.add_sub_input(left_y_axis);
    inputs.add_letter_input(a_button);

    


    let mut events = context.event_pump().unwrap();
    let mut m = InputAdapter::new_mouse();
    let mut c = InputAdapter::new_controller(inputs, resolutions);


    // loop until we receive a QuitEvent
    'event : loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened
        for event in events.poll_iter() {
            match event {
                Event::ControllerAxisMotion{ axis, value: val, .. } => {
                    //Dead zone is the sensitivity of the joystick
                    devices::handle_controller_axis(&mut c, axis, val);
                },
                Event::ControllerButtonDown{ button, .. } => {
                    devices::handle_controller_button(&mut c, button);
                },
                Event::Quit{..} => break 'event,
                _=> continue
            }
        }

        // get a mouse state
        let state = events.mouse_state();

        devices::handle_mouse(&mut m, &state);

        std::thread::sleep(Duration::from_millis(100));
    }
}
