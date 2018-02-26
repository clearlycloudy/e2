extern crate mazth;

use std::str;
use std::clone::Clone;

use self::mazth::quat::Quat;
use implement::file::md5anim;

#[derive(Debug, Clone)]
pub struct RigJoint {
    pub _name: String,
    pub _parent: i64,
    pub _pos: [f32;3],
    pub _orient: Quat<f32>,
}

#[derive(Debug, Clone)]
pub struct PoseJoints {
    pub _joints: Vec< RigJoint >,
    // pub _bbox_lower: [f32;3], //todo
    // pub _bbox_upper: [f32;3],
}

#[derive(Debug, Clone)]
pub struct PoseCollection {
    pub _frames: Vec< PoseJoints >,
    pub _framerate: u64,
}

pub fn process( anim: & md5anim::Md5AnimRoot ) -> Result< PoseCollection, & 'static str > {
    let mut pc = PoseCollection {
        _frames: vec![],
        _framerate: 0u64,
    };
    for i in 0..anim._frames.len() { //frames and bounds length should be equal
        let pj = match process_posejoints( & anim._frames[i], & anim._bounds[i], & anim._hierarchy, & anim._baseframe ){
            Ok( posejoint ) => posejoint,
            Err(e) => return Err(e)
        };
        pc._frames.push( pj );
    }
    pc._framerate = anim._framerate;
    Ok( pc )
}

fn process_posejoints( f: & md5anim::Frame, _bbox: & md5anim::Bound, hier: & Vec< md5anim::JointHierarchy >, baseframe: & Vec< md5anim::FrameJoint > ) -> Result< PoseJoints, & 'static str > {
    //hierarchy and baseframe length should be equal
    let mut pj = PoseJoints {
        _joints: vec![],
        // _bbox_lower: [0f32;3],
        // _bbox_upper: [0f32;3],
    };
    for i in 0..hier.len() {
        let start = hier[i]._start_index;
        let flag = hier[i]._flags;
        //obtain rotation and position from baseframe
        let mut bf_pos = baseframe[i]._pos;
        let mut bf_rot = baseframe[i]._orient;
        let mut offset = 0;
        for j in 0..3 { //override data if required from flag
            if (flag & (1u64 << j)) > 0 {
                assert!( start + offset < f._data.len() as u64 );
                bf_pos[j] = f._data[ ( start + offset ) as usize ];
                offset += 1;
            }
        }
        for j in 0..3 {
            if (flag & (8u64 << j)) > 0 {
                assert!( start + offset < f._data.len() as u64 );
                bf_rot[j] = f._data[ ( start + offset ) as usize ];
                offset += 1;
            }
        }
        //compute rotation quaternion
        let bf_orient = Quat::<f32>::init_from_vals_auto_w( bf_rot[0], bf_rot[1], bf_rot[2] ).normalize();
        let parent_joint_index = hier[i]._parent;
        let rj = if parent_joint_index >= 0 { //need to chain transformation from parent joint
            assert!( parent_joint_index < pj._joints.len() as i64 ); //the referenced parent joint is required to be alrady processed
            let parent_joint_frame = & pj._joints[ parent_joint_index as usize ];
            //update position and rotation
            let pos_quat = Quat::<f32>::init_from_vals_raw( bf_pos[0], bf_pos[1], bf_pos[2], 0. );
            let orient_inv = parent_joint_frame._orient.inverse().normalize();

            let res = parent_joint_frame._orient.mul( pos_quat ).mul( orient_inv );
            
            RigJoint {
                _name: hier[i]._name.clone(),
                _parent: hier[i]._parent,
                _pos: [ parent_joint_frame._pos[0] + res._x,
                        parent_joint_frame._pos[1] + res._y,
                        parent_joint_frame._pos[2] + res._z, ],
                _orient: parent_joint_frame._orient.mul( bf_orient ).normalize(),
            }
        }else{ //root joint
            RigJoint {
                _name: hier[i]._name.clone(),
                _parent: hier[i]._parent,
                _pos: bf_pos,
                _orient: bf_orient,
            }
        };
        pj._joints.push( rj );
    }
    Ok( pj )
}
