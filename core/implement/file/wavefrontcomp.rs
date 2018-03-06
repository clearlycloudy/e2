#![allow(non_snake_case)]
#![allow(unused_variables)]

extern crate pretty_env_logger;
extern crate nom;

use std::str;
use std::str::FromStr;

use self::nom::digit;

use interface::i_wavefront::{ obj, compute };

pub fn process( input: & obj::Collection ) -> Result< compute::ComputeCollection, & 'static str > {

    let mut batch_vert = vec![];
    let mut batch_normal = vec![];
    let mut batch_tc = vec![];

    for i in input._groups.iter() {
        for j in i._faces.iter() {
            for k in 0..3 {
                let v_idx = j._vert_index[ k ] - 1;
                if let None = j._normal_index {
                    panic!();
                }
                let n_idx = j._normal_index.unwrap()[ k ] - 1;

                if let None = j._tc_index {
                    panic!();
                }
                let tc_idx = j._tc_index.unwrap()[ k ] - 1;

                let v = &i._verts[ v_idx ][..];
                let n = &i._vert_normals[ n_idx ][..];
                let tc = &i._texture_coords[ tc_idx ][..];
                
                batch_vert.extend_from_slice( v );
                batch_normal.extend_from_slice( n );
                batch_tc.extend_from_slice( tc );
            }
        }
    }

    Ok(
        compute::ComputeCollection {
            _batch_vert: batch_vert,
            _batch_normal: batch_normal,
            _batch_tc: batch_tc,
        }
    )
}
