extern crate os_type; //added os_type crate to get information on operating system
use spat_input::InputAdapter;

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
#[derive(Debug)]
pub enum State{
    MoveMouse,
    LeftButtonDown,
    RightButtonDown,
    MiddleButtonDown,
    Idle
}


//These are traits that deal with the types of events based on states
//Will remain unimplemented for now
pub trait Event{
    fn move_mouse(&mut self, x:i32, y:i32);
    fn left_button_down();
    fn right_button_down();
    fn middle_button_down();
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
            m_button: Button::Middle,
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
    }

    fn left_button_down() {
        //Trying to differentiate based on operating system
        
        // let os = os_type::current_platform();

        // if os.os_type == "Ubuntu".unwrap() {
        //     print!("Left button pressed");
        //     print!("System is: {:?}", os.os_type);
        // }
        println!("left button pressed");
    }
    fn right_button_down() {
        println!("right button pressed");
    }
    fn middle_button_down() {
        println!("middle button pressed");
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
                _=> "Button down"
            },
        });

        self.output = format!("{}",out.to_string());
    }
}
