extern crate mazth;


///obj file format
pub mod obj {

    #[derive(Debug, Clone)]
    pub struct Face {
        pub _vert_index: [ usize; 3 ],
        pub _tc_index: Option< [ usize; 3 ] >,
        pub _normal_index: Option< [ usize; 3 ] >,
    }
    
    #[derive(Debug, Clone)]
    pub struct Group {
        pub _name: Option<String>,
        pub _group: Option<String>,
        pub _material: Option<String>,
        pub _verts: Vec< [ f32; 3 ] >,
        pub _vert_normals: Vec< [ f32; 3] >,
        pub _faces: Vec< Face >,
        pub _texture_coords: Vec< [ f32; 2 ] >,
    }

    #[derive(Debug, Clone)]
    pub struct Collection {
        pub _mtllib: String,
        pub _groups: Vec< Group >,
    }

}

pub mod compute {

    #[derive(Debug, Clone)]
    pub struct ComputeCollection {

        // pub _bbox_lower: [f32;3],
        // pub _bbox_upper: [f32;3],

        pub _batch_vert: Vec< f32 >,
        pub _batch_normal: Vec< f32 >,
        pub _batch_tc: Vec< f32 >,
    }
}
