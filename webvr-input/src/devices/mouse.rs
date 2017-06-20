use spat_input::InputAdapter;
use spat_input::ConnectionMode;
use std::collections::HashSet;
use std::time::{Duration,Instant};
use std::thread::sleep;


//TODO: Move this to a util or other subclass
//This struct represents the different types of inputs that a device can have
//as well as the min/max. This will be left to the developer to determine
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
    l_button: bool,
    r_button: bool,
    m_button: bool,
}

//The different states of the mouse
//TODO: Add middle scroll
#[derive(Debug, Clone)]
pub enum State{
    MoveMouse,
    LeftButtonDown,
    RightButtonDown,
    MiddleButtonDown,
    LeftRight,
    LeftMiddle,
    RightMiddle,
    Idle
}


//These are traits that deal with the different functions to be applied
//in order to generate different outputs
pub trait MouseResolutions {
    fn update_output(&mut self);
    fn move_mouse(&mut self, x:i32, y:i32);
    fn right_middle_up(&mut self);
    fn left_button_down(&mut self);
    fn left_button_up(&mut self);
    fn right_button_down(&mut self);
    fn right_button_up(&mut self);
    fn middle_button_down(&mut self);
    fn middle_button_up(&mut self);
    fn left_middle_down(&mut self);
    fn left_middle_up(&mut self);
    fn left_right_down(&mut self);
    fn left_right_up(&mut self);
    fn right_middle_down(&mut self);
}



impl Input {
    pub fn new() -> Input{
        Input {
            x:SubInput::new(),
            y:SubInput::new() ,
            l_button: false,
            r_button: false,
            m_button: false
        }
    }
}


//This is where the predefined structs and enums above are used to represent the device
//based on the generic input adapter from spat_input

impl InputAdapter<ConnectionMode, Input, State, String, Resolution, Event> {
    pub fn new_mouse() -> InputAdapter<ConnectionMode, Input, State, String, Resolution, Event> {
        InputAdapter{
            connection: ConnectionMode::SDL,
            input: Input::new(),
            state: State::Idle,
            output: "".to_string(),
            resolution: Resolution::new(),
            events: Event::new()
        }
    }
}


impl MouseResolutions for InputAdapter<ConnectionMode, Input, State, String, Resolution, Event> {
    fn update_output(&mut self) {
        let out = json!({
            "current_x": self.input.x.current,
            "current_y": self.input.y.current,
            "state": match self.state{
                State::Idle => "Idle",
                State::LeftButtonDown=> "Left Button down",
                State::RightButtonDown=> "Right Button down",
                State::MiddleButtonDown=> "Middle Button down",
                State::LeftMiddle=> "Left and Middle Button down",
                State::LeftRight=> "Left and Right Button down",
                State::RightMiddle=> "Right and Middle Button down",
                _=> "Combo Breaker"
            },
        });

        self.output = format!("{}",out.to_string());
    }

    fn move_mouse(&mut self, x:i32, y:i32) {
        self.input.x.current = x;
        self.input.y.current = y;
        self.state = State::MoveMouse;
    }

    fn right_middle_up(&mut self) {
        self.state = State::Idle;
        self.input.r_button = false;
        self.input.m_button = false;
        println!("right and middle released");
    }
    fn left_button_down(&mut self) {
        self.state = State::LeftButtonDown;
        self.input.l_button = true;
        println!("left button pressed");
    }

    fn left_button_up(&mut self) {
        self.state = State::Idle;
        self.input.l_button = false;
        println!("left button not pressed");
    }

    fn right_button_down(&mut self) {
        self.state = State::RightButtonDown;
        self.input.r_button = true;
        println!("right button pressed");
    }

    fn right_button_up(&mut self) {
        self.state = State::Idle;
        self.input.r_button = false;
        println!("right button not pressed");
    }

    fn middle_button_down(&mut self) {
        self.state = State::RightButtonDown;
        self.input.m_button = true;
        println!("middle button pressed");
    }

    fn middle_button_up(&mut self) {
        self.state = State::Idle;
        self.input.r_button = false;
        println!("middle button not pressed");
    }

    fn left_middle_down(&mut self) {
        self.state = State::LeftMiddle;
        self.input.l_button = true;
        self.input.m_button = true;
        println!("Left and middle down");
    }

    fn left_middle_up(&mut self) {
        self.state = State::LeftMiddle;
        self.input.l_button = false;
        self.input.m_button = false;
        println!("Left and middle released");
    }

    fn left_right_down(&mut self) {
        self.state = State::LeftRight;
        self.input.l_button = true;
        self.input.r_button = true;
        println!("Left and right down");
    }

    fn left_right_up(&mut self) {
        self.state = State::Idle;
        self.input.l_button = false;
        self.input.r_button = false;
        println!("Left and right released");
    }

    fn right_middle_down(&mut self) {
        self.state = State::RightMiddle;
        self.input.r_button = true;
        self.input.m_button = true;
        println!("right and middle down");
    }
}
