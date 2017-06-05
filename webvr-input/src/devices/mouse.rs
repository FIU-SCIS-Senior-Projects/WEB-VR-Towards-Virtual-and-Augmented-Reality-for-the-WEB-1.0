use spat_input::InputAdapter;
use std::collections::HashSet;
use std::time::{Duration,Instant};
use std::thread::sleep;


//Buttons used to represent the different physical buttons on the mouse and used for
//pattern matching
#[derive(Debug, Clone)]
pub enum Button{
    Left = 1,
    Right = 2,
    Middle = 3
}


//The different inputs available from the device
#[derive(Debug, Clone)]
pub struct Manipulation{
    x: i32,
    y: i32,
    l_button: Button,
    r_button: Button,
    m_button: Button, //TODO: implement middle scroll
}

//Struct containing the ranges of each manipulator in the device
#[derive(Debug, Copy, Clone)]
pub struct Input{
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
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


//These are traits that deal with the types of events based on states
//Will remain unimplemented for now
pub trait Event{
    fn move_mouse(&mut self, x:i32, y:i32);
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
    fn right_middle_up(&mut self);
}

//These are traits that deal with the different functions to be applied
//in order to generate different outputs
pub trait MouseResolutions {
    fn update_output(&mut self);
}


impl Manipulation {
    pub fn new() -> Manipulation{
        Manipulation {
            x: 0,
            y: 0,
            l_button: Button::Left,
            r_button: Button::Right,
            m_button: Button::Middle
        }
    }
}

impl Input {
    pub fn new() -> Input{
        Input {
            x_min: 0, //Ideally this should be screen size min
            x_max: 1280,
            y_min: 0, //Ideally this should be screen size min
            y_max: 720,
            l_button: false,
            r_button: false,
            m_button: false,
        }
    }
}

impl Event for Manipulation {
    fn move_mouse(&mut self, x:i32, y:i32) {

        self.x = x;
        self.y = y;
        let mut curr_state = State::MoveMouse;
        let mut curr_time = sleep(Duration::new(1,0));

        //check if the mouse finally stopped moving and is not holding anything before changing state to idle
        // if self.x == x && self.y == y && curr_time == sleep(Duration::new(1, 0)){
        //     let mut curr_state = State::Idle;
        //     println!("Mouse stopped moving");
        // }
    }

    fn left_button_down(&mut self) {
        let mut curr_state = State::LeftButtonDown;
        Input::new().l_button = true;
        println!("left button pressed");
    }

    fn left_button_up(&mut self) {
        let mut curr_state = State::Idle;
        Input::new().l_button = false;
        println!("left button not pressed");
    }

    fn right_button_down(&mut self) {
        let mut curr_state = State::RightButtonDown;
        Input::new().r_button = true;
        println!("right button pressed");
    }

    fn right_button_up(&mut self) {
        let mut curr_state = State::Idle;
        Input::new().r_button = false;
        println!("right button not pressed");
    }

    fn middle_button_down(&mut self) {
        let mut curr_state = State::MiddleButtonDown;
        Input::new().m_button = true;
        println!("middle button pressed");
    }

    fn middle_button_up(&mut self) {
        let mut curr_state = State::Idle;
        Input::new().m_button = false;
        println!("middle button not pressed");
    }

    fn left_middle_down(&mut self) {
        let mut curr_state = State::LeftMiddle;
        Input::new().l_button = true;
        Input::new().m_button = true;
        println!("Left and middle down");
    }

    fn left_middle_up(&mut self) {
        let mut curr_state = State::Idle;
        Input::new().l_button = false;
        Input::new().m_button = false;
        println!("Left and middle released");
    }

    fn left_right_down(&mut self) {
        let mut curr_state = State::LeftRight;
        Input::new().l_button = true;
        Input::new().r_button = true;
        println!("Left and right down");
    }

    fn left_right_up(&mut self) {
        let mut curr_state = State::Idle;
        Input::new().l_button = false;
        Input::new().r_button = false;
        println!("Left and right released");
    }

    fn right_middle_down(&mut self) {
        let mut curr_state = State::RightMiddle;
        Input::new().r_button = true;
        Input::new().m_button = true;
        println!("right and middle down");
    }

    fn right_middle_up(&mut self) {
        let mut curr_state = State::Idle;
        Input::new().r_button = true;
        Input::new().m_button = true;
        println!("right and middle released");
    }
}



//This is where the predefined structs and enums above are used to represent the device
//based on the generic input adapter from spat_input

impl InputAdapter<Manipulation, Input, String, State> {
    pub fn new_mouse() -> InputAdapter<Manipulation, Input, String, State> {
        InputAdapter{
            manipulation: Manipulation::new(),
            input: Input::new(),
            output: "".to_string(),
            state: State::Idle,
        }
    }
}

impl MouseResolutions for InputAdapter<Manipulation, Input, String, State> {
    fn update_output(&mut self) {
        let out = json!({
            "current_x": self.manipulation.x,
            "current_y": self.manipulation.y,
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
}
