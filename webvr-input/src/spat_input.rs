//! This serves as the initial representation of the 7-tuple to represent the main
//! devices. Few items will not be represented as they will be handled in main



//For future implementations regarding the types of connections that developers
//want to establish. This enum can be extended
#[derive(Debug, Clone)]
pub enum ConnectionMode {
    ClientServer,
    ExternInput,
}


//This will serve as a generic adapter to be used by all devices
#[derive(Debug, Copy, Clone)]
pub struct InputAdapter<T,U,V,W,X>{
    manipulation: T, //The different inputs on a device
    input: U, //The range of each input button 0..1
    output: V, //The ouput after applying a function based on device
    state: W, //defined enum for the different states of the device
    event: X, //event types based on the state of the device
}
