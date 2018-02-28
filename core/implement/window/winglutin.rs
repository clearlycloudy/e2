extern crate gl;
extern crate glutin;

use std::ops::FnMut;

use self::glutin::GlContext;

use interface::i_window::IWindow;

pub struct WinGlutinBase {
    pub _eventsloop: glutin::EventsLoop,
}

pub struct WinGlutinWin {
    pub _wingl: glutin::GlWindow,
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
        let c = glutin::ContextBuilder::new().with_vsync( true ).with_gl( gl_request );
        let wb = glutin::WindowBuilder::new().with_dimensions( w as u32, h as u32 );
        let base = WinGlutinBase {
            _eventsloop: glutin::EventsLoop::new(),
        };
        let w = WinGlutinWin {
            _wingl: glutin::GlWindow::new( wb, c, &base._eventsloop ).unwrap(),
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
}
