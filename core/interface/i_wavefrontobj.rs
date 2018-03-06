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
    pub struct Obj {
        pub _name: String,
        pub _material: String,
        pub _verts: Vec< [ f32; 3 ] >,
        pub _vert_normals: Vec< [ f32; 3] >,
        pub _faces: Vec< Face >,
        pub _texture_coords: Vec< [ f32; 2 ] >,
    }

    #[derive(Debug, Clone)]
    pub struct ObjCollection {
        pub _mtllib: String,
        pub _objs: Vec< Obj >,
    }

}
