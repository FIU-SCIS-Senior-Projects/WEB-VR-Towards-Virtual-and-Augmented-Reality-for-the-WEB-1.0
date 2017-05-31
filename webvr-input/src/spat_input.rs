//! This serves as the initial representation of the 7-tuple to represent the main
//! devices. Few items will not be represented as they will be handled in main
//! 
//! At this time, connection, events, and resolution function are not represenetd
//! in this adapter. The function for each device will be implemented with each
//! device as oppose to being defined here generically. Further, the function that
//! each device use may change depending on the manipulation that is being used.
//! The main connection logic will also be handled inside of the each device as 
//! different devices may have different needs. Lastly, the events are represented
//! as traits of the manipulation. That is an adapter can have a `device.move_mouse(x,y)`,
//! instead of storing these events, we will use Rust traits to easily modify the 
//! different events based on pattern matching with the states. This is also how
//! the different resolution functions are represented



//For future implementations regarding the types of connections that developers
//want to establish. This enum can be extended
#[derive(Debug, Clone)]
pub enum ConnectionMode {
    ClientServer,
    ExternInput,
}


//This will serve as a generic adapter to be used by all devices
#[derive(Debug, Copy, Clone)]
pub struct InputAdapter<T,U,V,W>{
    pub manipulation: T, //The different inputs on a device
    pub input: U, //The range of each input button 0..1
    pub output: V, //The ouput after applying a function based on device
    pub state: W, //defined enum for the different states of the device
}
