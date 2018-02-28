extern crate mazth;

#[cfg(test)]
#[allow(unused_imports)]
use std::ops::Div;
#[allow(unused_imports)]
use std::cmp::Ordering;

#[allow(unused_imports)]
use self::mazth::mat::Mat4x1;
#[allow(unused_imports)]
use interface::i_interpolate::IInterpolate;

use implement::math::spline_bezier::SplineBezier;
use implement::math::piecewise::Piecewise;

use test::math::test_interpolate;

#[test]
fn test_spline(){

    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
    let cp2 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    let cp3 = Mat4x1 { _val: [ 0f64, 1f64, -2f64, -3f64 ] };
    
    let mut spline = SplineBezier::init( 10, cp0, cp1, cp2, cp3 );

    test_interpolate::test_interp_forward( & mut spline, & cp0, & cp3 );
}

#[test]
fn test_spline_iterator(){
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
    let cp2 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    let cp3 = Mat4x1 { _val: [ 0f64, 1f64, -2f64, -3f64 ] };

    let mut spline = SplineBezier::init( 10, cp0, cp1, cp2, cp3 );
    let count_interpolated = spline.num_steps();
    assert!( count_interpolated == 10 );

    test_interpolate::test_iterator( & mut spline, & cp0, & cp3 );
}

//todo
// #[test]
// fn test_spline_reverse_iterator(){
//     let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
//     let cp1 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    
//     let mut interp = Spline::init( 10, cp0, cp1 );
//     let count_interpolated = interp.num_steps();
//     assert!( count_interpolated == 10 );

//     test_interpolate::test_reverse_iterator( & mut interp, & cp0, & cp1 );
// }


#[test]
fn test_spline_piecewise(){
    let mut splines = Piecewise::init();
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
    let cp2 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    let cp3 = Mat4x1 { _val: [ 0f64, 1f64, -2f64, -3f64 ] };
    for x in 0..4 {
        let spline = SplineBezier::init( 10 * (x+1) , cp0, cp1, cp2, cp3 );
        splines.add( spline );
    }
    let piece_count = 4usize;
    let total_steps = 100u64;
    test_interpolate::test_piecewise( & mut splines, & cp0, & cp3, piece_count, total_steps );
}

#[test]
fn test_spline_piecewise_bezier_iterator(){
    let mut splines = Piecewise::init();
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
    let cp2 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    let cp3 = Mat4x1 { _val: [ 0f64, 1f64, -2f64, -3f64 ] };
    for x in 0..4 {
        let spline = SplineBezier::init( 10 * (x+1) , cp0, cp1, cp2, cp3 );
        splines.add( spline );
    }

    let piece_count = 4usize;
    let total_steps = 100u64;
    test_interpolate::test_piecewise_iterator( & mut splines, & cp0, & cp3, piece_count, total_steps );
}

#[test]
fn test_spline_piecewise_reset(){
    let mut splines = Piecewise::init();
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
    let cp2 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    let cp3 = Mat4x1 { _val: [ 0f64, 1f64, -2f64, -3f64 ] };
    for x in 0..4 {
        let spline = SplineBezier::init( 10 * (x+1) , cp0, cp1, cp2, cp3 );
        splines.add( spline );
    }
    let piece_count = 4usize;
    let total_steps = 100u64;
    test_interpolate::test_piecewise_reset( & mut splines, & cp0, & cp3, piece_count, total_steps );
}
