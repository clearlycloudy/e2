extern crate mazth;

use std::str;

use self::mazth::{ mat::Mat3x1, quat::Quat };
use implement::file::md5rig;
use implement::file::md5mesh;

#[derive(Debug, Clone)]
pub struct VertCompute {
    pub _pos: [f32;3],
    pub _normal: [f32;3],
}

pub type Tri = md5mesh::Md5Tri;

#[derive(Debug, Clone)]
pub struct MeshCompute {
    pub _verts: Vec< VertCompute >,
    pub _tris: Vec< Tri >,
}

#[derive(Debug, Clone)]
pub struct ComputeCollection {
    pub _meshcomputes: Vec< MeshCompute >,
    pub _bbox_lower: [f32;3],
    pub _bbox_upper: [f32;3],
}

pub fn process( pc: & md5rig::PoseCollection, m: & md5mesh::Md5MeshRoot, pose_index_start: u64, pose_index_end: u64, interp: f32 ) -> Result< ComputeCollection, & 'static str > {
    if pose_index_start > pc._frames.len() as u64 {
        return Err( "pose_index_start out of bounds." )
    }
    if pose_index_end > pc._frames.len() as u64 {
        return Err( "pose_index_start out of bounds." )
    }
    let interp_clamped = if 0f32 > interp {
                             0f32
                         } else { if 1f32 < interp {
                             1f32
                         }else{
                             interp
                         } };
    let start = &pc._frames[ pose_index_start as usize ];
    let end = &pc._frames[ pose_index_end as usize ];
    interpolate( m, start, end, interp_clamped )
}

pub fn interpolate( m: & md5mesh::Md5MeshRoot, pose_start: & md5rig::PoseJoints, pose_end: & md5rig::PoseJoints, interp: f32 ) -> Result< ComputeCollection, & 'static str > {
    let mut cc = ComputeCollection {
        _meshcomputes: vec![],
        _bbox_lower: [0f32;3],
        _bbox_upper: [0f32;3],
    };

    for i in &m._meshes {
        let mut mc = MeshCompute {
            _verts: vec![],
            _tris: vec![],
        };
        for j in &i._verts {
            let mut vc = VertCompute {
                _pos: [0f32;3],
                _normal: [0f32;3],
            };
            for k in 0..j._weight_count {
                let weight_index = j._weight_start + k;
                let w = &i._weights[ weight_index as usize ];
                let joint_index = w._joint_index;
                if joint_index >= pose_start._joints.len() as u64 {
                    return Err( "joint index out of bounds in pose_start." )
                }
                if joint_index >= pose_end._joints.len() as u64 {
                    return Err( "joint index out of bounds in pose_start." )
                }
                let pose_start_rigjoint = & pose_start._joints[ joint_index as usize ];
                let pose_end_rigjoint = & pose_end._joints[ joint_index as usize ];
                //get position of the weight after transformation with joint orientation
                let pos_quat = Quat::<f32>::init_from_vals( w._pos[0], w._pos[1], w._pos[2], 0f32 );
                let orient_interp = Quat::<f32>::interpolate_slerp( pose_start_rigjoint._orient, pose_end_rigjoint._orient, interp );
                let orient_inv = orient_interp.inverse().normalize();
                let pos_transform = pose_start_rigjoint._orient.mul( pos_quat ).mul( orient_inv );
                //sum comtribution of weights for vertex position and vertex normal
                vc._pos[0] += ( pose_start_rigjoint._pos[0] + pos_transform._x ) * w._weight_bias;
                vc._pos[1] += ( pose_start_rigjoint._pos[1] + pos_transform._y ) * w._weight_bias;
                vc._pos[2] += ( pose_start_rigjoint._pos[2] + pos_transform._z ) * w._weight_bias;
            }
            mc._verts.push( vc );
        }
        //calculate vertex normal via cross product
        for j in &i._tris {
            let v0_index = j._vert_indices[ 0 ];
            let v1_index = j._vert_indices[ 1 ];
            let v2_index = j._vert_indices[ 2 ];
            if v0_index >= mc._verts.len() as u64 {
                return Err( "vert0 index out of bounds" )
            }
            if v1_index >= mc._verts.len() as u64 {
                return Err( "vert1 index out of bounds" )
            }
            if v2_index >= mc._verts.len() as u64 {
                return Err( "vert2 index out of bounds" )
            }
            let v0 = Mat3x1 {
                _val: mc._verts[ v0_index as usize ]._pos,
            };
            let v1 = Mat3x1 {
                _val: mc._verts[ v1_index as usize ]._pos,
            };
            let v2 = Mat3x1 {
                _val: mc._verts[ v2_index as usize ]._pos,
            };
            let v01 = v1.minus( &v0 ).unwrap();
            let v02 = v2.minus( &v0 ).unwrap();
            let n = v02.cross( &v01 ).expect("cross product for vertex normal invalid")
                       .normalize().expect("normalize for vertex normal invalid");
            for k in 0..3 {
                mc._verts[ v0_index as usize ]._normal[ k ] = n._val[ k ];
                mc._verts[ v1_index as usize ]._normal[ k ] = n._val[ k ];
                mc._verts[ v2_index as usize ]._normal[ k ] = n._val[ k ];
            }
        }
        mc._tris.extend_from_slice( &i._tris[..] );
        cc._meshcomputes.push( mc );
        // for h in 0..3 {
        //     cc._bbox_lower[h] = pose_start._bbox_lower[h];
        //     cc._bbox_upper[h] = pose_start._bbox_upper[h];
        // }
    }

    let mut max_pos = [0., 0., 0.];
    let mut min_pos = [0., 0., 0.];
    for i in cc._meshcomputes.iter() {
        for j in i._tris.iter() {
            for k in 0..3 {
                let idx_vert = j._vert_indices[ k ];
                let vert = & i._verts[ idx_vert as usize ];
                for h in 0..3 {
                    if max_pos[h] < vert._pos[h] {
                        max_pos[h] = vert._pos[h];
                    }
                    if min_pos[h] > vert._pos[h] {
                        min_pos[h] = vert._pos[h];
                    }
                }
            }
        }
    }
    cc._bbox_lower = min_pos;
    cc._bbox_upper = max_pos;
    Ok( cc )
}
