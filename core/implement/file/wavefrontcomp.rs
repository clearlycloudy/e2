#![allow(non_snake_case)]
#![allow(unused_variables)]

extern crate pretty_env_logger;
extern crate nom;

use std::str;
use std::str::FromStr;
use std::f32;

use self::nom::digit;

use interface::i_wavefront::{ obj, compute };

pub fn process( input: & obj::Collection ) -> Result< compute::ComputeCollection, & 'static str > {

    let mut batch_vert = vec![];
    let mut batch_normal = vec![];
    let mut batch_tc = vec![];
    let mut bbox_upper = [ 0f32; 3 ];
    let mut bbox_lower = [ 0f32; 3 ];

    let mut verts = vec![];
    let mut normals = vec![];
    let mut texture_coords = vec![];
    
    input._groups.iter().for_each( |x| {
        verts.extend_from_slice( &x._verts[..] );
        normals.extend_from_slice( &x._vert_normals[..] );
        texture_coords.extend_from_slice( &x._texture_coords[..] );
    } );

    for i in input._groups.iter() {
        for j in i._faces.iter() {
            for k in 0..3 {
                let v_idx = j._vert_index[ k ] - 1;
                if let None = j._normal_index {
                    return Err( "normal index not present")
                }
                let n_idx = j._normal_index.unwrap()[ k ] - 1;


                let tc = match j._tc_index {
                    Some(x) => {
                        let tc_idx = j._tc_index.unwrap()[ k ] - 1;
                        assert!( tc_idx < texture_coords.len() );
                        &texture_coords[ tc_idx ][..]
                    },
                    None => {
                        &[ 0., 0. ][..]
                    }
                    // return Err( "texture coord index not present")
                };

                if v_idx >= verts.len() {
                    return Err( "vertex index out of range" )
                }
                assert!( v_idx < verts.len() );
                assert!( n_idx < normals.len() );
                
                let v = &verts[ v_idx ][..];
                let n = &normals[ n_idx ][..];
                
                for l in 0..3 {
                    bbox_upper[l] = bbox_upper[l].max( v[l] );
                    bbox_lower[l] = bbox_lower[l].min( v[l] );
                }
                
                batch_vert.extend_from_slice( v );
                batch_normal.extend_from_slice( n );
                batch_tc.extend_from_slice( tc );
            }
        }
    }

    Ok(
        compute::ComputeCollection {
            _bbox_upper: bbox_upper,
            _bbox_lower: bbox_lower,
            _batch_vert: batch_vert,
            _batch_normal: batch_normal,
            _batch_tc: batch_tc,
        }
    )
}
