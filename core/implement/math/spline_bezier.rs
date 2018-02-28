extern crate mazth;

use std::cmp;

use interface::i_interpolate::*;

use self::mazth::mat::*;

#[allow(unused_variables)]
#[derive(Debug)]
#[derive(Clone)]
pub struct SplineBezier {
    pub _ctl: [ Mat4x1< f64 >; 4 ],
    pub _point: Mat4x1< f64 >,
    pub _point_d: Mat4x1< f64 >,
    pub _point_dd: Mat4x1< f64 >,
    pub _point_ddd: Mat4x1< f64 >,
    pub _basis: Mat4< f64 >,
    pub _steps: i64,
    pub _step_current: i64,
}

impl SplineBezier {
    pub fn init( s: u64, cp0: Mat4x1< f64 >, cp1: Mat4x1< f64 >, cp2: Mat4x1< f64 >, cp3: Mat4x1< f64 > ) -> SplineBezier {
        assert!( s > 2 );
        let d = 1f64 / ( s as f64 - 1f64 );
        let d2 = d * d;
        let d3 = d * d2;
        
        let px = Mat1x4 { _val: [ cp0._val[0], cp1._val[0], cp2._val[0], cp3._val[0] ] };
        let py = Mat1x4 { _val: [ cp0._val[1], cp1._val[1], cp2._val[1], cp3._val[1] ] };
        let pz = Mat1x4 { _val: [ cp0._val[2], cp1._val[2], cp2._val[2], cp3._val[2] ] };
        let pw = Mat1x4 { _val: [ cp0._val[3], cp1._val[3], cp2._val[3], cp3._val[3] ] };
        
        let basis = Mat4 { _val: [ 1f64, -3f64, 3f64, -1f64,
                                   0f64, 3f64, -6f64, 3f64,
                                   0f64, 0f64, 3f64, -3f64,
                                   0f64, 0f64, 0f64, 1f64 ],
                           _is_row_major: true
        };
        
        let mut cvec: [ Mat1x4<_>; 4 ] = [ Mat1x4 { _val: [0f64; 4] } ; 4 ];
        cvec[0] = px.mul_mat4x4( &basis ).expect("mat1x4 mul mat4x4 invalid");
        cvec[1] = py.mul_mat4x4( &basis ).expect("mat1x4 mul mat4x4 invalid");
        cvec[2] = pz.mul_mat4x4( &basis ).expect("mat1x4 mul mat4x4 invalid");
        cvec[3] = pw.mul_mat4x4( &basis ).expect("mat1x4 mul mat4x4 invalid");
        
        let mut dq : Mat4x1<_> = Mat4x1::default();
        let mut ddq : Mat4x1<_> = Mat4x1::default();
        let mut dddq : Mat4x1<_> = Mat4x1::default();

        for i in 0..4 {
            let a = cvec[i]._val[3]; //t^3 term
            let b = cvec[i]._val[2]; //t^2
            let c = cvec[i]._val[1]; //t^1
            dq._val[i] = a * d3 + b * d2 + c * d;
            ddq._val[i] = 6f64 * a * d3 + 2f64 * b * d2;
            dddq._val[i] = 6f64 * a * d3;
        }
        
        SplineBezier {
                        _ctl: [ cp0, cp1, cp2, cp3 ],
                        _point: cp0, //start point
                        _point_d: dq,
                        _point_dd: ddq,
                        _point_ddd: dddq,
                        _basis: basis,
                        _steps: s as _,
                        _step_current: -1i64,
        }
    }
}
#[allow(unused_variables)]
impl IInterpolate< Mat4x1< f64 > > for SplineBezier {
    // type DataType = Mat4x1< f64 >;
    fn num_steps( & self ) -> u64 {
        self._steps as u64
    }
    fn interp_delta( & mut self, steps: i64 ) -> Option< Mat4x1< f64 > > {
        if steps >= 0 {
            for x in 0..steps {
                
                self._step_current = cmp::min( self._step_current + 1, self._steps );
                
                if self._step_current == self._steps {
                    self._point = self._ctl[3];
                    return Some( self._point )
                }
                
                if self._step_current == 0 {
                    continue;
                }

                self._point = self._point.plus( & self._point_d ).expect("point plus invalid");
                self._point_d = self._point_d.plus( & self._point_dd ).expect("point d plus invalid");
                self._point_dd = self._point_dd.plus( & self._point_ddd ).expect("point dd plus invalid");
            }
            return Some( self._point )
        } else {
            unimplemented!();
        }
    }
    fn interp_current( & self ) -> Mat4x1< f64 > {
        self._point
    }
    fn interp_is_end( & self ) -> bool {
        self._step_current == self._steps
    }
    fn interp_is_start( & self ) -> bool {
        self._step_current == -1
    }
    fn reset( & mut self ){
        *self = SplineBezier::init( self._steps as _, self._ctl[0], self._ctl[1], self._ctl[2], self._ctl[3] );
    }
}

//required by IInterpolate
impl Iterator for SplineBezier {
    type Item = Mat4x1< f64 >;
    fn next( & mut self ) -> Option< Mat4x1< f64 > > {
        self._step_current = cmp::min( self._step_current + 1, self._steps );
        if self._step_current == self._steps {
            return None
        } else {
            if self._step_current == 0 {
                self._point = self._ctl[0];
                return Some( self._point )
            } else {
                self._point = self._point.plus( & self._point_d ).expect("point plus invalid");
                self._point_d = self._point_d.plus( & self._point_dd ).expect("point d plus invalid");
                self._point_dd = self._point_dd.plus( & self._point_ddd ).expect("point dd plus invalid");
                return Some( self._point )
            }
        }
    }
}

//required by IInterpolate
impl DoubleEndedIterator for SplineBezier {
    fn next_back( & mut self ) -> Option< Mat4x1< f64 > > {
        unimplemented!();
    }
}

