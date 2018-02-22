extern crate mazth;

use self::mazth::mat::{ Mat2x1, Mat3x1 };
use self::mazth::quat::Quat;

// use std::f32::consts::PI;

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
    // pub fn start_motion( & mut self, pos: & Mat2x1<f32> ){
    //     self._pos_last = self.project_cursor_to_hemisphere( pos );
    // }
    pub fn move_motion( & mut self, pos_start: & Mat2x1<f32>, pos: & Mat2x1<f32> ){

        self._pos_last = self.project_cursor_to_hemisphere( pos_start );
        
        let pos_current = self.project_cursor_to_hemisphere( pos );
        
        let delta = pos_current.minus( &self._pos_last ).unwrap();
        
        if delta[0].abs() > 0.001 || delta[1].abs() > 0.001 || delta[2].abs() > 0.001 {
            let angle = ( self._pos_last.dot( & pos_current ).unwrap()
                          / ( self._pos_last.magnitude().unwrap() * pos_current.magnitude().unwrap() ) )
                .acos();

            let axis = self._pos_last.cross( & pos_current ).unwrap().normalize().unwrap();

            let q = Quat::<f32>::init_from_axis_angle_radian( ( axis, 3. * angle ) ).normalize();

            // println!( "axis: {:?}", axis );
            // println!( "angle: {}", angle );

            self._rot = self._rot.mul( q ).normalize();
        }
    }
    fn project_cursor_to_hemisphere( & self, pos: & Mat2x1<f32> ) -> Mat3x1<f32> {

        let mut p : Mat3x1<f32> = Default::default();
        let r = self._w.min( self._h ) * 1.5; //radius of the trackball
        p[0] = ( pos[0] - self._w / 2. ) / r; //normalize
        p[1] = ( pos[1] - self._h / 2. ) / r; //normalize
        let mut d = p.magnitude().unwrap();
        if d > 1. {
            p[0] = p[0] / (d * d);
            p[1] = p[1] / (d * d);
            d = 1.;
        }
        let elevation = ( 1. - d * d ).sqrt();
        p[2] = elevation;
        p
    }
    pub fn get_rot( & self ) -> & Quat<f32> {
        & self._rot
    }
    pub fn reset_rot( & mut self ) {
        self._rot = Default::default();
    }
}
