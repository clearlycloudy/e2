extern crate mazth;

use interface::i_interpolate::*;

use self::mazth::mat::*;

// use implement::render::camera;

use self::mazth::mat::Mat4x1;
use self::mazth::i_comparable::IComparableError;

use interface::i_interpolate::IInterpolate;

use interface::i_step::Step;

use interface::i_waypoint::IWaypoint;

use implement::step::step_interpolate;

use implement::math::spline_bezier::SplineBezier;

use implement::math::piecewise::Piecewise;

pub struct CamGuide {
    pub _pos_trajectory: Option< Piecewise< SplineBezier, Mat4x1<f64> > >,
}

impl IWaypoint for CamGuide {
    fn set_next( & mut self, pos: Mat4x1<f64> ){
        unimplemented!();
        // match self._pos_trajectory {
        //     None => {
        //         let mut splines = Piecewise::init();
        //         let cp0 = pos;
        //         // let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
        //         // let cp2 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
        //         // let cp3 = Mat4x1 { _val: [ 0f64, 1f64, -2f64, -3f64 ] };
        //         let spline = SplineBezier::init( 10 , cp0, cp1, cp2, cp3 );
        //         splines.add( spline );
        //         self._pos_trajectory = splines;
        //     },
        //     Some(x) => {
        //         // let cp0 = pos;
        //         // // let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
        //         // // let cp2 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
        //         // // let cp3 = Mat4x1 { _val: [ 0f64, 1f64, -2f64, -3f64 ] };
        //         // let spline = SplineBezier::init( 10 , cp0, cp1, cp2, cp3 );
        //         // splines.add( spline );
        //     },
        // }
    }
    fn get_trajectory( & mut self ) -> & mut Option< Piecewise< SplineBezier, Mat4x1<f64> > > {
        & mut self._pos_trajectory
    }
    fn flush( & mut self ){
        self._pos_trajectory = None;
    }
}
