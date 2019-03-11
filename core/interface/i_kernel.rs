extern crate chrono;

use self::chrono::prelude::*;

use std::fmt::Debug;

use interface::i_window::IWindow;
use interface::i_game_logic::IGameLogic;
use interface::i_renderer::IRenderer;
use interface::i_ui::IUi;

pub trait IKernel < W: IWindow,
                    I: IUi< EventInput = W::EventType >,
                    G: IGameLogic< EventInput = I::EventInputFiltered >,
                    R: IRenderer< EventRender = G::EventRender > >
    : AsMut< W > + AsMut< I > + AsMut< G > + AsMut< R > where I::EventInputFiltered : Debug

//possibly use trait bounds and delegate traits to subfields in concrete implementer when this is supported by Rust (https://github.com/rust-lang/rfcs/pull/1406)
    // IWindow +
    // IGameLogic< EventInput = < Self as IWindow >::EventType > +
    // IRenderer< EventRender = < Self as IGameLogic >::EventRender >
{
    fn new() -> Result< Self, & 'static str > where Self: Sized;

    fn new_with < F > ( f: F ) -> Result< Self, & 'static str >
        where F: FnOnce() -> Result< Self, & 'static str >, Self: Sized
    {
        f()
    }

    fn init_hook( & mut self ) -> Result< (), & 'static str > { Ok( () ) }

    fn deinit_hook( & mut self ) -> Result< (), & 'static str > { Ok( () ) }

    ///default implementation of the main control flow
    fn run( & mut self ) -> Result< (), & 'static str > {

        self.init_hook()?;
            
        info!( "kernel running." );
        
        //foever loop and process results until exit conditions are caught
        let mut running = true;

        #[allow(unused_mut)]
        let mut sigs_for_window = vec![];

        let mut t_cycle_last = Local::now();

        while running {

            let t0 = Local::now();

            //process windowing events into buffer
            (self.as_mut() as & W).make_current()?;

            (self.as_mut() as & mut W).per_frame_setup()?;

            (self.as_mut() as & mut W).handle_signal_request( sigs_for_window.as_slice() )?;

            let mut events_window : Vec< W::EventType > = vec![];
            match (self.as_mut() as & mut W).handle_events_pass_thru() {
                Some( x ) => {
                    events_window.push( x );
                },
                _ => {},
            }

            let win_offset = (self.as_mut() as & mut W).get_offset().expect("window offset invalid");
            let win_size = (self.as_mut() as & mut W).get_size().expect("window size invalid");
            
            // info!( "win offset: {:?}, win size: {:?}", win_offset, win_size );
            
            let events_inputs_filtered = (self.as_mut() as & mut I).process_input_events( events_window.as_slice(), win_offset, win_size );

            let t1 = Local::now();
            
            let ( events_render, signal_exit ) : ( Vec< _ >, bool ) = (self.as_mut() as & mut G).process_input_events( events_inputs_filtered.as_slice(), win_offset, win_size );

            if signal_exit {
                running = false;
            }

            let t2 = Local::now();
            
            (self.as_mut() as & mut R).process_render_events( events_render ).is_ok();

            (self.as_mut() as & mut W).swap_buf();

            let t3 = Local::now();

            let t_1_0 = t1.signed_duration_since(t0).num_microseconds().unwrap() as f64;
            let t_2_1 = t2.signed_duration_since(t1).num_microseconds().unwrap() as f64;
            let t_3_2 = t3.signed_duration_since(t2).num_microseconds().unwrap() as f64;
            let t_cycle = t0.signed_duration_since(t_cycle_last).num_microseconds().unwrap() as f64;
            t_cycle_last = t0;
            debug!( "t lapse ui input filter: {} ms", t_1_0 / 1000. );
            debug!( "t lapse game logic: {} ms", t_2_1 / 1000. );
            debug!( "t lapse renderer: {} ms", t_3_2 / 1000. );
            if t_cycle > 0. {
                info!( "frame rate: {} Hz", 1_000_000. / t_cycle );
            }
        }
        
        info!( "kernel shutdown." );

        self.deinit_hook()?;

        Ok( () )
    }
}
