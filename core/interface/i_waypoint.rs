extern crate mazth;

use self::mazth::mat::Mat4x1;

use implement::math::spline_bezier::SplineBezier;

use implement::math::piecewise::Piecewise;

pub trait IWaypoint {
    fn set_next( & mut self, pos: Mat4x1<f64> );
    fn get_trajectory( & mut self ) -> & mut Option< Piecewise< SplineBezier, Mat4x1<f64> > >; 
    fn flush( & mut self );
}
