extern crate mazth;

#[allow(unused_imports)]
use std::ops::Div;
#[allow(unused_imports)]
use std::cmp::Ordering;

#[allow(unused_imports)]
use self::mazth::mat::Mat4x1;
#[allow(unused_imports)]
use interface::i_interpolate::IInterpolate;

use implement::math::linearinterp::LinearInterp;
use implement::math::piecewise::Piecewise;

use test::math::test_interpolate;

#[test]
fn test_linearinterp(){
        
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };

    {
        let mut interp = LinearInterp::init( 10, cp0, cp1 );
        test_interpolate::test_interp_forward( & mut interp, & cp0, & cp1 );
    }
    {
        let mut interp = LinearInterp::init( 10, cp0, cp1 );
        test_interpolate::test_interp_forward_back( & mut interp, & cp0, & cp1 );
    }
}

#[test]
fn test_linearinterp_iterator(){
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    
    let mut interp = LinearInterp::init( 10, cp0, cp1 );
    let count_interpolated = interp.num_steps();
    assert!( count_interpolated == 10 );

    test_interpolate::test_iterator( & mut interp, & cp0, & cp1 );
}

#[test]
fn test_linearinterp_reverse_iterator(){
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    
    let mut interp = LinearInterp::init( 10, cp0, cp1 );
    let count_interpolated = interp.num_steps();
    assert!( count_interpolated == 10 );

    test_interpolate::test_reverse_iterator( & mut interp, & cp0, & cp1 );
}


#[test]
fn test_linearinterp_piecewise(){
    let mut interps = Piecewise::init();
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
    for x in 0..4 {
        let interp = LinearInterp::init( 10 * (x+1) , cp0, cp1 );
        interps.add( interp );
    }
    let piece_count = 4usize;
    let total_steps = 100u64;
    test_interpolate::test_piecewise( & mut interps, & cp0, & cp1, piece_count, total_steps );
}


#[test]
fn test_linearinterp_piecewise_reset(){
    let mut interps = Piecewise::init();
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
    for x in 0..4 {
        let interp = LinearInterp::init( 10 * (x+1) , cp0, cp1 );
        interps.add( interp );
    }
    let piece_count = 4usize;
    let total_steps = 100u64;
    test_interpolate::test_piecewise_reset( & mut interps, & cp0, & cp1, piece_count, total_steps );
}
