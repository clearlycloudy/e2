extern crate mazth;

///md5mesh file format
pub mod mesh {

    use super::mazth::quat::Quat;
    
    #[derive(Debug, Clone)]
    pub struct Md5Mesh {
        pub _shader: String,
        pub _numverts: u64,
        pub _numtris: u64,
        pub _numweights: u64,
        pub _verts: Vec< Md5Vert >,
        pub _tris: Vec< Md5Tri >,
        pub _weights: Vec< Md5Weight >,
    }

    #[derive(Debug, Clone)]
    pub struct Md5Joint {
        pub _name: String,
        pub _parent_index: i64,
        pub _pos: [f32;3],
        pub _orient: [f32;3],
        pub _rot: Quat<f32>,
    }

    #[derive(Debug, Clone)]
    pub struct Md5Vert {
        pub _index: u64,
        pub _tex_coords: [f32;2],
        pub _weight_start: u64,
        pub _weight_count: u64,
        pub _normal: [f32;3],
        pub _pos: [f32;3],
    }

    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct Md5Tri {
        pub _index: u64,
        pub _vert_indices: [u64;3],
    }

    #[derive(Debug, Clone)]
    pub struct Md5Weight {
        pub _index: u64,
        pub _joint_index: u64,
        pub _weight_bias: f32,
        pub _pos: [f32;3],
    }

    #[derive(Debug, Clone)]
    pub struct Md5MeshRoot {
        pub _md5ver: u64,
        pub _cmdline: String,
        pub _numjoints: u64,
        pub _nummeshes: u64,
        pub _joints: Vec< Md5Joint >,
        pub _meshes: Vec< Md5Mesh >,
    }

    impl Md5MeshRoot {
        pub fn init() -> Md5MeshRoot {
            Md5MeshRoot {
                _md5ver: 0u64,
                _cmdline: String::from(""),
                _numjoints: 0u64,
                _nummeshes: 0u64,
                _joints: vec![],
                _meshes: vec![],
            }
        }
    }
}

///md5anim file format
pub mod anim {

    #[derive(Debug)]
    pub struct JointHierarchy {
        pub _name: String,
        pub _parent: i64,
        pub _flags: u64,
        pub _start_index: u64,
    }

    #[derive(Debug)]
    pub struct Bound {
        pub _min: [f32;3],
        pub _max: [f32;3],
    }

    #[derive(Debug)]
    pub struct FrameJoint {
        pub _index: u64,
        pub _pos: [f32;3],
        pub _orient: [f32;3],
    }

    #[derive(Debug)]
    pub struct Frame {
        pub _index: u64,
        pub _data: Vec< f32 >,
    }

    #[derive(Debug)]
    pub struct Md5AnimRoot {
        pub _md5ver: u64,
        pub _cmdline: String,
        pub _numframes: u64,
        pub _numjoints: u64,
        pub _framerate: u64,
        pub _num_animated_components: u64,
        pub _hierarchy: Vec< JointHierarchy >,
        pub _bounds: Vec< Bound >,
        pub _baseframe: Vec< FrameJoint >,
        pub _frames: Vec< Frame >,
    }

    impl Md5AnimRoot {
        pub fn init() -> Md5AnimRoot {
            Md5AnimRoot {
                _md5ver: 0u64,
                _cmdline: String::from(""),
                _numframes: 0u64,
                _numjoints: 0u64,
                _framerate: 0u64,
                _num_animated_components: 0u64,
                _hierarchy: vec![],
                _bounds: vec![],
                _baseframe: vec![],
                _frames: vec![],
            }
        }
    }
}

///md5rig file format
pub mod rig {

    use super::mazth::quat::Quat;
    
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
}

pub mod compute {
    ///md5compute format
    #[derive(Debug, Clone)]
    pub struct VertCompute {
        pub _pos: [f32;3],
        pub _normal: [f32;3],
    }

    #[derive(Debug, Clone)]
    pub struct MeshCompute {
        pub _verts: Vec< VertCompute >,
    }

    #[derive(Debug, Clone)]
    pub struct ComputeCollection {
        // pub _meshcomputes: Vec< MeshCompute >, //use batch instead
        pub _bbox_lower: [f32;3],
        pub _bbox_upper: [f32;3],

        pub _batch_vert: Vec< f32 >,
        pub _batch_normal: Vec< f32 >,
        pub _batch_tc: Vec< f32 >,
    }
}
