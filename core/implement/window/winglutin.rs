extern crate gl;
extern crate glutin;

use std::ops::FnMut;

use self::glutin::Context;
use self::glutin::dpi::{LogicalSize,PhysicalSize,PhysicalPosition};
use self::glutin::ContextTrait;

use interface::i_window::IWindow;

pub struct WinGlutinBase {
    pub _eventsloop: glutin::EventsLoop,
}

pub struct WinGlutinWin {
    pub _wingl: glutin::WindowedContext,
}

pub struct WinGlutin {
    pub _base: WinGlutinBase,
    pub _win: WinGlutinWin,
}

pub struct DummySignalRequestType {
}

impl IWindow for WinGlutin {

    type EventType = glutin::Event;
    type SignalRequestType = DummySignalRequestType;

    fn new( w: u64, h: u64 ) -> WinGlutin {
        let gl_request = glutin::GlRequest::Latest;

        let wb = glutin::WindowBuilder::new().with_dimensions(
            LogicalSize::from( (w as u32,
                                h as u32) ) );

        let base = WinGlutinBase {
            _eventsloop: glutin::EventsLoop::new(),
        };
        
        let c = glutin::ContextBuilder::new()
            .with_vsync( true )
            .with_gl( gl_request )
            .build_windowed( wb, &base._eventsloop )
            .unwrap();
        
        // let w = WinGlutinWin {
        //     _wingl: glutin::Window::new( wb, c, &base._eventsloop ).unwrap(),
        // };
        
        // let w = WinGlutinWin {
        //     _wingl: wb.build( &base._eventsloop ).unwrap(),
        // };

        let w = WinGlutinWin {
            _wingl: c, //windowed context
        };
        
        WinGlutin {
            _base: base,
            _win: w,
        }
    }
    fn make_current( & self ) -> Result< (), & 'static str > {
        unsafe {
            self._win._wingl.make_current().unwrap();
        }
        gl::load_with( |symbol| self._win._wingl.get_proc_address(symbol) as * const _ );
        Ok( () )
    }
    fn handle_events < F > ( & mut self, cb: F ) -> ()
        where F : FnMut( Self::EventType ) -> () {
        self._base._eventsloop.poll_events( cb );
        ()
    }
    fn handle_events_pass_thru( & mut self ) -> Option< Self::EventType > {
        //todo: handle specific events as requested via handle_signal_request
        let mut e = None;
        self._base._eventsloop.poll_events( |event| {
            e = Some(event);
            ()
        } );
        e
    }
    fn swap_buf( & self ) -> () {
        self._win._wingl.swap_buffers().unwrap();
        ()
    }
    fn handle_signal_request( & mut self, _sig: & [ Self::SignalRequestType ] ) -> Result< (), & 'static str > {
        //todo
        Ok( () )
    }
    fn per_frame_setup( & mut self ) -> Result< (), & 'static str > {
        unsafe {
            gl::ClearColor( 0.9, 0.9, 0.9, 1.0 );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        Ok( () )
    }

    fn get_offset( & self ) -> Option<(i32,i32)> {
        match self._win._wingl.get_position() {
            Some( logical_pos ) => {
                // let dpi = self._win._wingl.get_current_monitor().get_hidpi_factor();
                let dpi = self._win._wingl.get_hidpi_factor();
                Some( logical_pos.to_physical( dpi ).into() )
                // Some( logical_pos.into() )
            },
            _ => None,
        }
    }
    
    fn get_size( & self ) -> Option<(u32,u32)> {
        match self._win._wingl.get_inner_size() {
            Some( logical_size ) => {
                // let dpi = self._win._wingl.get_hidpi_factor();
                let dpi = self._win._wingl.get_current_monitor().get_hidpi_factor();
                Some( logical_size.to_physical( dpi ).into() )                
                // Some( logical_size.into() )
            },
            _ => None,
        }
    }

}
