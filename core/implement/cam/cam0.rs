use interface::i_bound::{ IBound, BoundType };
use interface::i_interpolate::*;

use implement::math::mat::*;
use implement::math::frustum::*;

pub struct Cam0 {
    // : IInterpolate< Mat4x1< f64 >, Item = Mat4x1< f64 > >
    _frustum: Frustum,
}

impl Default for Cam0 {
    fn default() -> Cam0 {
        Cam0 {
            _frustum: unimplemented!(),            
        }
    }
}

impl Cam0 {
    pub fn init(  ) -> Cam0 {
        Cam0 {
            _frustum: unimplemented!(),
        }
    }
    pub fn set_focus( target: &IBound ) -> bool {
        unimplemented!();
    }
    pub fn set_position< T >( target: Mat4x1< f64 > ) -> bool {
        unimplemented!();
    }
    pub fn set_trans_focus( transition_param: u32 ) -> bool { //transition properties
        unimplemented!();
    }
    pub fn set_trans_trajectory( transition_param: u32) -> bool { //transition properties
        unimplemented!();
    }
    pub fn update_cycle(){ //update camera settings by fetching the necessary data inputs
        unimplemented!();
    }
    pub fn set_trajectory_auto() -> bool {
        //make trajectory offset by an amount away from focus, potentially circling the focus target
        unimplemented!();
    }
    pub fn set_trajectory_off() -> bool {
        unimplemented!();
    }
    pub fn rotate( rot: u32 ) { //disables trajectory and rotate by an input quaternion or euler angle
        unimplemented!();
    }
}