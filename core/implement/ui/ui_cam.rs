extern crate mazth;

use self::mazth::mat;

use implement::cam::trackball::TrackBall;

use interface::i_ui::{ InputFiltered, KeyCode, State, Coord };

pub struct UiCam {
    pub _trackball: TrackBall,
    pub _mouse_r_down: bool,
    pub _mouse_pos_down: (f32,f32),
    pub _mouse_pos: (f32,f32),
    pub _move: (isize, isize, isize),
}

impl Default for UiCam {
    fn default() -> Self {
        Self {
            _trackball: Default::default(),
            _mouse_r_down: false,
            _mouse_pos_down: ( 0., 0.),
            _mouse_pos: ( 0., 0. ),
            _move: ( 0, 0, 0 ),
        }
    }
}

impl UiCam {
    pub fn process( & mut self, i: & InputFiltered, win_offset: (i32,i32), win_size: (u32,u32) ) {
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
            &InputFiltered::Button { key: KeyCode::W, state: State::Press } => {
                self._move.0 += 1;
                info!( "W pressed" );
            },
            &InputFiltered::Button { key: KeyCode::S, state: State::Press } => {
                self._move.0 -= 1;
                info!( "S pressed" );
            },
            &InputFiltered::Button { key: KeyCode::A, state: State::Press } => {
                self._move.1 -= 1;
                info!( "A pressed" );
            },
            &InputFiltered::Button { key: KeyCode::D, state: State::Press } => {
                self._move.1 += 1;
                info!( "D pressed" );
            },
            &InputFiltered::Button { key: KeyCode::Z, state: State::Press } => {
                self._move.2 -= 1;
                info!( "Z pressed" );
            },
            &InputFiltered::Button { key: KeyCode::X, state: State::Press } => {
                self._move.2 += 1;
                info!( "X pressed" );
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
                            
                            self._trackball.move_motion( & old_mouse_pos, & new_mouse_pos, win_size );
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
                            
                            self._trackball.move_motion( & old_mouse_pos, & new_mouse_pos, win_size );
                        }
                    },
                    _ => {}
                }
            },
            &InputFiltered::MouseCoord2( x, y) => {

                let old_mouse_pos = mat::Mat2x1 { _val: [ self._mouse_pos.0,
                                                          self._mouse_pos.1
                ] };

                self._mouse_pos = (x,y);

                if self._mouse_r_down {

                    let new_mouse_pos = mat::Mat2x1 { _val: [ self._mouse_pos.0,
                                                              self._mouse_pos.1
                    ] };
                    
                    self._trackball.move_motion( & old_mouse_pos, & new_mouse_pos, win_size );
                }
            },
            _ => {
                // info!( "{:?}", _ );
            },
        }        
    }
}

