#![cfg(test)]
#![allow(unused_imports)]

extern crate mazth;

use self::mazth::mat::Mat4x1;
use self::mazth::i_comparable::IComparableError;


use interface::i_interpolate::IInterpolate;

use interface::i_step::Step;

use implement::step::step_interpolate;

use implement::math::spline_bezier::SplineBezier;
use implement::math::piecewise::Piecewise;

#[test]
fn test_step_interpolate(){

    let mut splines = Piecewise::init();
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
    let cp2 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    let cp3 = Mat4x1 { _val: [ 0f64, 1f64, -2f64, -3f64 ] };
    for _ in 0..2 {
        let spline = SplineBezier::init( 10 , cp0, cp1, cp2, cp3 );
        splines.add( spline );
    }
    assert!(splines._pieces.len() == 2 );

    let mut s = Step {
        _current_val: 10.0,
        _range_val: ( 10.0, 20.0 ),
    };

    let step_delta = 5f64;

    {
        step_interpolate::step_delta( & mut s, step_delta, & mut splines );

        let val = splines.interp_current();

        assert!( val.is_equal( &cp3, 0.00001f64 ).expect("is_equal invalid") );
    }

    {
        let val = step_interpolate::step_delta( & mut s, step_delta, & mut splines );


        assert!( val.is_equal( &cp3, 0.00001f64 ).expect("is_equal invalid") );
    }
}

#[test]
fn test_step_interpolate_clamp(){

    let mut splines = Piecewise::init();
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
    let cp2 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    let cp3 = Mat4x1 { _val: [ 0f64, 1f64, -2f64, -3f64 ] };
    for _ in 0..2 {
        let spline = SplineBezier::init( 10 , cp0, cp1, cp2, cp3 );
        splines.add( spline );
    }
    assert!(splines._pieces.len() == 2 );

    let mut s = Step {
        _current_val: 10.0,
        _range_val: ( 10.0, 20.0 ),
    };

    let step_delta = 15f64;

    step_interpolate::step_delta( & mut s, step_delta, & mut splines );

    let val = splines.interp_current();

    assert!( val.is_equal( &cp3, 0.00001f64 ).expect("is_equal invalid") );
}

#[test]
fn test_step_interpolate_start(){

    let mut splines = Piecewise::init();
    let cp0 = Mat4x1 { _val: [ 0f64, 1f64, 2f64, 3f64 ] };
    let cp1 = Mat4x1 { _val: [ 5f64, 6f64, 7f64, 8f64 ] };
    let cp2 = Mat4x1 { _val: [ 10f64, 16f64, 17f64, 18f64 ] };
    let cp3 = Mat4x1 { _val: [ 0f64, 1f64, -2f64, -3f64 ] };
    for _ in 0..2 {
        let spline = SplineBezier::init( 10 , cp0, cp1, cp2, cp3 );
        splines.add( spline );
    }
    assert!(splines._pieces.len() == 2 );

    let mut s = Step {
        _current_val: 10.0,
        _range_val: ( 10.0, 20.0 ),
    };

    let step_delta = 0f64;

    step_interpolate::step_delta( & mut s, step_delta, & mut splines );

    let val = splines.interp_current();

    assert!( val.is_equal( &cp0, 0.00001f64 ).expect("is_equal invalid") );
}
