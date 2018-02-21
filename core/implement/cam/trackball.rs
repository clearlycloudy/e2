extern crate mazth;

use self::mazth::mat::{ Mat2x1, Mat3x1 };
use self::mazth::quat::Quat;

use std::f32::consts::PI;

pub struct TrackBall {
    _rot: Quat<f32>,
    _pos_last: Mat3x1<f32>,
    _w: f32,
    _h: f32,
}

impl Default for TrackBall {
    fn default() -> TrackBall {
        TrackBall {
            _rot: Default::default(),
            _pos_last: Default::default(),
            _w: 100.,
            _h: 100.,
        }
    }
}

impl TrackBall {
    pub fn new( w: f32, h: f32 ) -> TrackBall {
        TrackBall {
            _rot: Default::default(),
            _pos_last: Default::default(),
            _w: w,
            _h: h,
        }
    }
    pub fn set_w_h( & mut self, w: f32, h: f32 ){
        assert!( w > 0. );
        assert!( h > 0. );
        self._w = w;
        self._h = h;
    }
    pub fn start_motion( & mut self, pos: & Mat2x1<f32> ){
        self._pos_last = self.project_cursor_to_hemisphere( pos );
    }
    pub fn move_motion( & mut self, pos: & Mat2x1<f32> ){

        let pos_current = self.project_cursor_to_hemisphere( pos );
        
        let delta = pos_current.minus( &self._pos_last ).unwrap();
        
        if delta[0].abs() > 0.001 || delta[1].abs() > 0.001 || delta[2].abs() > 0.001 {
            let angle = ( self._pos_last.dot( & pos_current ).unwrap()
                          / ( self._pos_last.magnitude().unwrap() * pos_current.magnitude().unwrap() ) )
                .acos() * 180. / PI;

            let axis = self._pos_last.cross( & pos_current ).unwrap().normalize().unwrap();
            
            self._pos_last = pos_current;

            let q = Quat::<f32>::init_from_axis_angle_degree( ( axis, angle ) );

            // println!( "axis: {:?}", axis );
            // println!( "angle: {}", angle );

            self._rot = self._rot.mul( q ).normalize();
        }
    }
    fn project_cursor_to_hemisphere( & self, pos: & Mat2x1<f32> ) -> Mat3x1<f32> {
        let mut p : Mat3x1<f32> = Default::default();
        
        p[0] = ( 2. * pos[0] - self._w )/ self._w;
        p[1] = ( self._h - 2. * pos[1] )/ self._h;

        let d = p.magnitude().unwrap();

        p[2] = ( PI / 2. * (if d < 1. { d } else { 1. }) ).cos();
        
        p.normalize().unwrap()
    }
    pub fn get_rot( & self ) -> & Quat<f32> {
        & self._rot
    }
    pub fn reset_rot( & mut self ) {
        self._rot = Default::default();
    }
}
