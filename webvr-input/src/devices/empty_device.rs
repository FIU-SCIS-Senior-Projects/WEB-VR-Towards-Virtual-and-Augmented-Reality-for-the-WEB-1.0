use spat_input::InputAdapter;
use std::collections::HashSet;

//the generic or empty device needs to be parsed in order to create the tuple as needed.
//we will need to use the serde_json library in order to parse and create the enum as needed
##[derive(Debug, Clone)]
pub enum Buttons {
    Button1 = 1,
    Button2 = 2,
    Button3 = 3,
    Button4 = 4,
    Button5 = 5
}
