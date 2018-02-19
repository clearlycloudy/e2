extern crate mazth;

#[allow(unused_imports)]
use std::ops::Div;
#[allow(unused_imports)]
use std::cmp::Ordering;

use self::mazth::i_comparable::IComparableError;
use self::mazth::mat::{Mat3x1, Mat4};
use implement::math::util;

#[test]
fn test_math_util(){

    //look_at
    {
        let eye : Mat3x1<f32> = Mat3x1 { _val: [5.0,5.0,5.0] };
        let center : Mat3x1<f32> = Mat3x1 { _val: [0.0,0.0,0.0] };
        let up : Mat3x1<f32> = Mat3x1 { _val: [0.0,1.0,0.0] };

        let lookat = util::look_at( eye, center, up );
        assert!( lookat.is_equal( &Mat4{ _val: [ 0.70711, 0.0, -0.70711, 0.0,
                                                 -0.40825, 0.81650, -0.40825, 0.0,
                                                 0.57735, 0.57735, 0.57735, -8.66025,
                                                 0.0, 0.0, 0.0, 1.0 ], _is_row_major: true }, 0.0001f32 ).expect("look_at result unexpected") );
    }

    //perspective transform
    {
        let fov = 90.0;
        let aspect = 1.0;
        let near = 0.1;
        let far = 100.0;
        let persp = util::perspective( fov, aspect, near, far );
        println!( "{:?}", persp );
        assert!( persp.is_equal( &Mat4{ _val: [ 1.0, 0.0, 0.0, 0.0,
                                                0.0, 1.0, 0.0, 0.0,
                                                0.0, 0.0, -1.0, -0.2,
                                                0.0, 0.0, -1.0, 0.0 ], _is_row_major: true }, 0.01f32 ).expect("perspective result unexpected") );
    }
}
