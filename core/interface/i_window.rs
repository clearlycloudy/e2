use std::ops::FnMut;

pub trait IWindow {
    
    type EventType;
    type SignalRequestType;
    
    fn new( dimx: u64, dimy: u64 ) -> Self;
    fn make_current( & self ) -> Result< (), & 'static str >;
    fn handle_events < F > ( & mut self, cb: F ) -> () where F: FnMut( Self::EventType ) -> ();
    fn handle_events_pass_thru( & mut self ) -> Option< Self::EventType >;
    fn swap_buf( & self ) -> ();
    fn handle_signal_request( & mut self, & [ Self::SignalRequestType ] ) -> Result< (), & 'static str >;
    fn per_frame_setup( & mut self ) -> Result< (), & 'static str >;
    fn get_offset( & self ) -> Option<(i32,i32)>;
    fn get_size( & self ) -> Option<(u32,u32)>;
}
