extern crate mazth;

use self::mazth::mat;

use implement::cam::trackball::TrackBall;

use interface::i_ui::{ InputFiltered, KeyCode, State, Coord };

pub struct UiCam {
    pub _trackball: TrackBall,
    pub _mouse_r_down: bool,
    pub _mouse_pos_down: (f32,f32),
    pub _mouse_pos: (f32,f32),
}

impl Default for UiCam {
    fn default() -> Self {
        Self {
            _trackball: Default::default(),
            _mouse_r_down: false,
            _mouse_pos_down: (0., 0.),
            _mouse_pos: (0., 0.),
        }
    }
}

impl UiCam {
    pub fn process( & mut self, i: & InputFiltered ) {
        match i {
            &InputFiltered::Button { key: KeyCode::MouseR, state: State::Press } => {
                self._mouse_r_down = true;
                self._mouse_pos_down = self._mouse_pos;
                info!( "mouse r down" );
            },
            &InputFiltered::Button { key: KeyCode::MouseR, state: State::Release } => {
                self._mouse_r_down = false;
                self._mouse_pos_down = self._mouse_pos;
                info!( "mouse r up" );
            },
            &InputFiltered::MouseCoord( c, v) => {
                match c {
                    Coord::X => {

                        let old_mouse_pos = mat::Mat2x1 { _val: [ self._mouse_pos.0,
                                                                  self._mouse_pos.1
                        ] };

                        self._mouse_pos.0 = v;
                        
                        if self._mouse_r_down {

                            let new_mouse_pos = mat::Mat2x1 { _val: [ self._mouse_pos.0,
                                                                      self._mouse_pos.1
                            ] };
                            
                            self._trackball.move_motion( & old_mouse_pos, & new_mouse_pos );
                        }
                    },
                    Coord::Y => {

                        let old_mouse_pos = mat::Mat2x1 { _val: [ self._mouse_pos.0,
                                                                  self._mouse_pos.1
                        ] };

                        self._mouse_pos.1 = v;

                        if self._mouse_r_down {

                            let new_mouse_pos = mat::Mat2x1 { _val: [ self._mouse_pos.0,
                                                                      self._mouse_pos.1
                            ] };
                            
                            self._trackball.move_motion( & old_mouse_pos, & new_mouse_pos );
                        }
                    },
                    _ => {}
                }
            },
            _ => {},
        }        
    }
}

