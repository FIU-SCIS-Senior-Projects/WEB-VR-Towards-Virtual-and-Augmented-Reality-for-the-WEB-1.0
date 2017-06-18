//For future implementations regarding the types of connections that developers
//want to establish. This enum can be extended. Connection can include libraries
//as well such as SDL, or LIBUSB or even Native APIs
#[derive(Debug, Clone)]
pub enum ConnectionMode {
    SDL,
    LibUSB,
    Native,
}


//This will serve as a generic adapter to be used by all devices
#[derive(Debug, Copy, Clone)]
pub struct InputAdapter<C,I,S,O,F,E>{
    pub connection: C, //Represents the connections of the device
    pub input: I, //The range of each input button 0..1
    pub state: S, //defined enum for the different states of the device
    pub output: O, //The ouput after applying a function based on device
    pub resolution: F, // The functions that are provided by the developers for each state
    pub events: E, //The events that the developer chooses to represent on each device
}
