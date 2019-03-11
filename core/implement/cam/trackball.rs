///create a trackball with a radius that is
///proportional to minimum of window width and height.

extern crate mazth;

use self::mazth::mat::{ Mat2x1, Mat3x1 };
use self::mazth::quat::Quat;

//use std::f32::consts::PI;

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
            _w: 300.,
            _h: 300.,
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
    ///updates the rotation quaternion based on input cursor positions
    pub fn move_motion( & mut self, pos_start: & Mat2x1<f32>, pos: & Mat2x1<f32>, win_size: (u32,u32) ){

        self._pos_last = self.project_cursor_to_hemisphere( pos_start, win_size );
        
        let pos_current = self.project_cursor_to_hemisphere( pos, win_size );
        
        let delta = pos_current.minus( &self._pos_last ).unwrap();

        if delta[0].abs() > 0.00001 || delta[1].abs() > 0.00001 || delta[2].abs() > 0.00001 {
            let mag_p_last = self._pos_last.magnitude().unwrap();
            let mag_p = pos_current.magnitude().unwrap();
            assert!( mag_p > 0.99 && mag_p < 1.01);
            assert!( mag_p_last > 0.99 && mag_p_last < 1.01);
            let angle = ( self._pos_last.dot( & pos_current ).unwrap()
                          / ( mag_p * mag_p_last ) )
                .acos();
        
            
            if angle.abs() > 0.00001 {

                let axis = self._pos_last.cross( & pos_current ).unwrap().normalize().unwrap();
                
                let q = Quat::<f32>::init_from_axis_angle_radian( ( axis, angle ) ).normalize();
                
                self._rot = self._rot.mul( q ).normalize();
            }
        }
    }
    ///project cursor position onto trackball sphere and normalize it to unit vector
    fn project_cursor_to_hemisphere( & self, pos: & Mat2x1<f32>, win_size: (u32,u32) ) -> Mat3x1<f32> {

        //issue: coordinates from joystick reading is not in the same units as screen units (logicalunits)
        let mut p : Mat3x1<f32> = Default::default();
        let r = win_size.0.min( win_size.1 ) as f32 * 0.5; //radius of the trackball
        p[0] = ( pos[0] - win_size.0 as f32 / 2. ) / r; //normalize, x-direction
        p[2] = ( pos[1] - win_size.1 as f32 / 2. ) / r; //normalize, z-direction, assumed same as camera up-vector
        
        let mut d = p.magnitude().unwrap();
        if d > 1. {
            p[0] = p[0] / d;
            p[2] = p[2] / d;
            d = 1.;
        }
        let elevation = ( 1. - d * d ).sqrt();
        p[1] = elevation; //y-direction
        // println!( "p: {:?}, win size: {:?}", p, win_size );
        p
    }
    pub fn get_rot( & self ) -> & Quat<f32> {
        & self._rot
    }
    ///resets rotation to none
    pub fn reset_rot( & mut self ) {
        self._rot = Default::default();
    }
}
