use spat_input::InputAdapter;


//Buttons used to represent the different physical buttons on the mouse and used for 
//pattern matching
#[derive(Debug, Clone)]
pub enum Button{
    Left,
    Right,
    Middle
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
pub struct State{}


//This struct deals with the types of events based on states
//Will remain unimplemented for now
#[derive(Debug, Copy, Clone)]
pub struct Event{

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

impl State {
    pub fn new() -> State {
        State{}
    }

    pub fn move_mouse(man: &mut Manipulation, x:i32, y:i32) {
        man.x = x;
        man.y = y;
    }

    pub fn left_button_down() {
        println!("left button pressed");
    }
    pub fn right_button_down() {
        println!("right button pressed");
    }
    pub fn middle_button_down() {
        println!("middle button pressed");
    }
}
impl Event {
    pub fn new() -> Event {
        Event{}
    }
}

//This is where the predefined structs and enums above are used to represent the device
//based on the generic input adapter from spat_input
impl InputAdapter<Manipulation, Input, String, State, Event> {
    pub fn new_mouse() -> InputAdapter<Manipulation, Input, String, State, Event> {
        InputAdapter{
            manipulation: Manipulation::new(),
            input: Input::new(),
            output: "Current Output".to_string(),
            state: State::new(),
            event: Event::new()
        }
    }
}
