use spat_input::{InputAdapter, ConnectionMode, SubInput };
use std::collections::HashSet;
use std::time::{Duration,Instant};
use std::thread::sleep;

impl SubInput<i32> {
    pub fn new_int(min:i32, max:i32) -> SubInput<i32>{
        SubInput {
            min: min,
            current: 0,
            max: max,
        }
    }
}


//The different inputs available from the device
#[derive(Debug, Clone)]
pub struct Event{
}

impl Event {
    pub fn new() -> Event {
        Event{

        }
    }
}

//resolution function to be applied to each input(to be specified by developer)
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
#[derive(Debug,Clone)]
pub struct Input{
    pub num_input_list: Vec<SubInput<i32>>,
    pub str_input_list: Vec<String>,
}

//The different states of the mouse
#[derive(Debug, Clone)]
pub enum State{
    Idle,
}


//These are traits that deal with the different functions to be applied
//in order to generate different outputs
pub trait ControllerResolutions {
    fn update_output(&mut self);
}



impl Input {
    pub fn new() -> Input{
        Input {
            num_input_list: Vec::new(),
            str_input_list: Vec::new()

        }
    }

    //Adds input to the list of inputs to be mapped with resolution functions
    pub fn add_sub_input(&mut self, input: SubInput<i32>) {
        self.num_input_list.push(input);
    }

    pub fn add_letter_input(&mut self, input: String) {
        self.str_input_list.push(input);

    }
}


//This is where the predefined structs and enums above are used to represent the device
//based on the generic input adapter from spat_input

impl InputAdapter<ConnectionMode, Input, State, String, Resolution, Event> {
    pub fn new_controller(inputs: Input, resolutions: Resolution) -> InputAdapter<ConnectionMode, Input, State, String, Resolution, Event> {
        InputAdapter{
            connection: ConnectionMode::SDL,
            input: inputs,
            state: State::Idle,
            output: "".to_string(),
            resolution: resolutions,
            events: Event::new()
        }
    }
}


impl ControllerResolutions for InputAdapter<ConnectionMode, Input, State, String, Resolution, Event> {
    fn update_output(&mut self) {
        let out = json!({
            "Input": self.input.str_input_list,
            "state": match self.state{
                State::Idle => "Idle",
                _=> "Combo Breaker"
            },
        });

        self.output = format!("{}",out.to_string());
    }

}
