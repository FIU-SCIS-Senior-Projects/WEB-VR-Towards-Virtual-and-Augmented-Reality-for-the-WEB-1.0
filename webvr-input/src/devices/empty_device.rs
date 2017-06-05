extern crate os_type;
use spat_input::InputAdapter;
use std::collections::HashSet;
use os_type::current_platform();

//the generic or empty device needs to be parsed in order to create the tuple as needed.
//we will need to use the serde_json library in order to parse and create the enum as needed
##[derive(Debug, Clone)]
pub enum Buttons {

}
