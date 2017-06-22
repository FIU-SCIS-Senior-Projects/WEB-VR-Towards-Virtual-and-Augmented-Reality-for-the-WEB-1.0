extern crate serde_json;
extern crate sdl2;
extern crate libusb;

use spat_input::InputAdapter;
use spat_input::ConnectionMode;
use std::collections::HashSet;

//Here is a function to have us get the device information as needed to build the struct
//Function should ask system what device is available and give properties in JSON
fn get_device_info() {
    let mut context = libusb::Context::new().unwrap();

    for mut device in context.devices().unwrap().iter() {
        let device_info = device.device_descriptor().unwrap();
        let device_name = device.address();
        let device_bus = device.bus_number();
        let device_buttons = device.open(); //This may need to be revised
        //println!("{0}", serde_json::to_string(&device_name).unwrap());
    }
}

//the generic or empty device needs to be parsed in order to create the tuple as needed.
//we will need to use the serde_json library in order to parse and create the enum as needed
struct unsupported_device {
    name: String,
    numButtons: i32, //I dont think that many devices will have more than 32-bits worth of buttons for now
}

//Button enum version 2
//Since we do not know much about this device, a simple button enumeration will help
//This means that there is only limited support for an empty device
//The best we can do is tell if a button has been pressed
#[derive(Debug, Clone)]
pub enum Buttons {
    AButton = 1 //support for one button at the present time. All other buttons on device routed through here.
}

//Generic device may not have many states. This will most likely vary depending on device.
//This will most likely change as more information gets pulled from device.
pub enum State{
    MoveDevice,
    DownActive, //generic device property pressed
    UpActive, //generic device property released
    Idle
}

//This was taken from mouse class
//This is a struct for inputs or subinputs in a window
//Then this is implemented with parameters for a new input

//TODO: Move this to a util or other subclass
#[derive(Debug, Copy, Clone)]
pub struct SubInput<T> {
    min: T,
    current: T,
    max: T,
}

impl SubInput<i32> {
    pub fn new() -> SubInput<i32>{
        SubInput {
            min: 0,
            current: 0,
            max: 9999,
        }
    }
}

//events available from generic device
#[derive(Debug, Clone)]
pub struct Event{
}

impl Event {
    pub fn new() -> Event {
        Event{

        }
    }
}

//resolution functions for empty device.
#[derive(Debug, Clone)]
pub struct Resolution{
}

impl Resolution {
    pub fn new() -> Resolution {
        Resolution{

        }
    }
}

//Struct containing the ranges of each manipulator in the device
#[derive(Debug, Copy, Clone)]
pub struct Input{
    x: SubInput<i32>,
    y: SubInput<i32>,
    z: SubInput<i32>,
    button_press: bool
}

//resolution functions for a generic device.
pub trait GenericResolutions {
    fn update_output(&mut self);
    fn move_device_pointer(&mut self, x:i32, y:i32, z:i32);
    fn button_down(&mut self);
    fn button_up(&mut self);
}

//input implementation
impl Input {
    pub fn new() -> Input{
        Input {
            x:SubInput::new(),
            y:SubInput::new(),
            z:SubInput::new(),
            button_press: false
        }
    }
}

//take all the pieces and use them to represent the generic device
//note much of this is borad because the device is not supported by this library.
impl GenericResolutions for InputAdapter<ConnectionMode, Input, State, String, Resolution, Event> {
    fn update_output(&mut self) {
        let out = json!({
            "current_x": self.input.x.current,
            "current_y": self.input.y.current,
            "current_z": self.input.z.current,
            "state": match self.state{
                State::Idle => "Idle",
                State::MoveDevice => "Device Movement Detected",
                State::DownActive => "Generic Button Down",
                State::UpActive => "Generic Button Up",
                _=> "Combo Breaker"
            },
        });

        self.output = format!("{}",out.to_string());
    }

    fn move_device_pointer(&mut self, x:i32, y:i32, z:i32){
        self.input.x.current = x;
        self.input.y.current = y;
        self.input.z.current = z;
        self.state = State::MoveDevice;
    }

    fn button_down(&mut self){
        self.state = State::DownActive;
        self.input.button_press = true;
        println!("Generic Button Pressed");
    }

    fn button_up(&mut self){
        self.state = State::UpActive;
        self.input.button_press = false;
        println!("Generic Button Released");
    }
}
