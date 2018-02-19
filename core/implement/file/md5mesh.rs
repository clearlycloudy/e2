extern crate mazth;

use std::str;
use std::collections::HashMap;
use std::str::FromStr;

use self::mazth::quat::Quat;
use implement::file::md5common;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub enum Token {
    Version,
    Commandline,
    Numjoints,
    Nummeshes,
    Joints,
    Mesh,
    Shader,
    Numverts,
    Vert,
    Numtris,
    Tri,
    Numweights,
    Weight,
}

#[derive(Debug)]
pub struct Md5Mesh {
    pub _shader: String,
    pub _numverts: u64,
    pub _numtris: u64,
    pub _numweights: u64,
    pub _verts: Vec< Md5Vert >,
    pub _tris: Vec< Md5Tri >,
    pub _weights: Vec< Md5Weight >,
}

#[derive(Debug)]
pub struct Md5Joint {
    pub _name: String,
    pub _parent_index: i64,
    pub _pos: [f32;3],
    pub _orient: [f32;3],
    pub _rot: Quat<f32>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Md5Weight {
    pub _index: u64,
    pub _joint_index: u64,
    pub _weight_bias: f32,
    pub _pos: [f32;3],
}

#[derive(Debug)]
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

pub fn process_joint( file_content: &str, idx: usize, hm: & HashMap< &str, Token > ) -> Result< ( usize, Md5Joint ), & 'static str > {
    let mut j = Md5Joint {
        _name: String::from(""),
        _parent_index: -1i64,
        _pos: [0f32;3],
        _orient: [0f32;3],
        _rot: Quat::<f32>::init()
    };
    let mut idx_current = idx;
    let ( tok, _kw_tok, idx_s, idx_e, idx_next ) = md5common::tokenize( &file_content[0..], idx_current, & hm );
    match tok {
        md5common::Token::Str => {
            j._name = String::from_str( &file_content[idx_s..idx_e] ).expect("md5mesh parse joint name failed");
            idx_current = idx_next;
        },
        _ => panic!("unexpected token. string not found.")
    }
    let ( tok, _kw_tok, idx_s, idx_e, idx_next ) = md5common::tokenize( &file_content[0..], idx_current, & hm );
    match tok {
        md5common::Token::Int => {
            j._parent_index = i64::from_str( &file_content[idx_s..idx_e] ).expect("md5mesh parse joint parent index failed");
            idx_current = idx_next;
        },
        _ => panic!("unexpected token. string not found.")
    }
    {
        match md5common::expect_parenl( &file_content[0..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
    }
    {
        match md5common::expect_float_array( &file_content[0..], idx_current, 3usize, & mut j._pos ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
    }
    {
        match md5common::expect_parenr( &file_content[0..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
    }
    {
        match md5common::expect_parenl( &file_content[0..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
    }
    {
        match md5common::expect_float_array( &file_content[0..], idx_current, 3usize, & mut j._orient ){
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e)
        }
    }
    {
        match md5common::expect_parenr( &file_content[0..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
    }
    Ok( ( idx_current, j ) )
}

pub fn process_mesh( file_content: &str, idx: usize, hm: & HashMap< &str, Token > ) -> Result< ( usize, Md5Mesh ), & 'static str > {
    let mut j = Md5Mesh {
        _shader: String::from(""),
        _numverts: 0u64,
        _numtris: 0u64,
        _numweights: 0u64,
        _verts: vec![],
        _tris: vec![],
        _weights: vec![],
    };
    let mut idx_current = idx;
    match md5common::expect_bracel( &file_content[0..], idx_current ) {
        Ok( v ) => { idx_current = v },
        Err( _ ) => { return Err( "unexpected token. bracel not present") }
    }
    loop {
        let ( tok, kw_tok, idx_s, idx_e, idx_next ) = md5common::tokenize( &file_content[0..], idx_current, & hm );
        match tok {
            md5common::Token::Keyword => {
                match kw_tok {
                    //expect: keyword: shader
                    Some(Token::Shader) => {
                        idx_current = idx_next;
                        //expect: string: shader path
                        match md5common::expect_str( &file_content[0..], idx_current, & hm, & mut j._shader ) {
                            Ok(v) => { idx_current = v; },
                            Err(e) => assert!(false, e )
                        }
                    },
                    //expect: keyword: numverts
                    Some(Token::Numverts) => {
                        idx_current = idx_next;
                        //expect: int: numverts
                        let mut arr: [i64;1] = [0i64;1];
                        match md5common::expect_int_array( &file_content[0..], idx_current, 1usize, & mut arr ) {
                            Ok(v) => idx_current = v,
                            Err(e) => assert!(false, e )
                        }
                        j._numverts = arr[0] as u64;
                        for _ in 0.. j._numverts {
                            //expect: keyword: vert
                            match process_vert( &file_content[0..], idx_current, & hm ) {
                                Ok( ( index, vert ) ) => {
                                    idx_current = index;
                                    j._verts.push(vert);
                                },
                                Err(e) => assert!(false, e )
                            }
                        }
                        println!("count verts: {:?}", j._verts.len() );
                    },
                    //expect: keyword: numtris
                    Some(Token::Numtris) => {
                        idx_current = idx_next;
                        //expect: int: numtris
                        let mut arr: [i64;1] = [0i64;1];
                        match md5common::expect_int_array( &file_content[0..], idx_current, 1usize, & mut arr ) {
                            Ok(v) => idx_current = v,
                            Err(e) => assert!(false, e )
                        }
                        j._numtris = arr[0] as u64;
                        for _ in 0.. j._numtris {
                            //expect: keyword: tri
                            match process_tri( &file_content[0..], idx_current, & hm ) {
                                Ok( ( index, tri ) ) => {
                                    idx_current = index;
                                    j._tris.push(tri);
                                },
                                Err(e) => assert!(false, e )
                            }
                        }
                        println!("count tris: {:?}", j._tris.len() );
                    },
                    //expect: keyword: numweights
                    Some(Token::Numweights) => {
                        idx_current = idx_next;
                        //expect: int: numweights
                        let mut arr: [i64;1] = [0i64;1];
                        match md5common::expect_int_array( &file_content[0..], idx_current, 1usize, & mut arr ) {
                            Ok(v) => idx_current = v,
                            Err(e) => assert!(false, e )
                        }
                        j._numweights = arr[0] as u64;
                        for _ in 0.. j._numweights {
                            //expect: keyword: weight
                            match process_weight( &file_content[0..], idx_current, & hm ) {
                                Ok( ( index, weight ) ) => {
                                    idx_current = index;
                                    j._weights.push(weight);
                                },
                                Err(e) => assert!(false, e )
                            }
                        }
                        println!("count weights: {:?}", j._weights.len() );
                    },
                    _ => panic!("unexpected token. shader keyword not found.")
                }
            },
            md5common::Token::Bracer => {
                return Ok( ( idx_next, j ) )
            },  
            _ => panic!("unexpected token '{:?}'. keyword not found.", &file_content[idx_s..idx_e] )
        }
    }
}

pub fn process_vert( file_content: &str, idx: usize, hm: & HashMap< &str, Token > ) -> Result< ( usize, Md5Vert ), & 'static str > {
    let mut idx_current = idx;
    let mut vert = Md5Vert {
        _index: 0u64,
        _tex_coords: [0f32;2],
        _weight_start: 0u64,
        _weight_count: 0u64,
        _normal: [0f32;3],
        _pos: [0f32;3],
    };
    let ( tok, kw_tok, _idx_s, _idx_e, idx_next ) = md5common::tokenize( &file_content[0..], idx_current, & hm );
    match tok {
        md5common::Token::Keyword => {
            match kw_tok {
                //expect: keyword: vert
                Some(Token::Vert) => {
                    idx_current = idx_next;
                },
                _ => return Err("unexpected token. Vert is not present.")
            }
        },
        _ => return Err("unexpected token. keyword is not present.")
    }
    let mut arr0 : [i64;1] = [0i64;1];
    match md5common::expect_int_array( &file_content[0..], idx_current, 1usize, & mut arr0 ) {
        Ok(v) => {
            idx_current = v;
            vert._index = arr0[0] as u64;
        },
        Err(e) => assert!(false, e )
    }
    match md5common::expect_parenl( &file_content[0..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    match md5common::expect_float_array( &file_content[0..], idx_current, 2usize, & mut vert._tex_coords ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    match md5common::expect_parenr( &file_content[0..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    let mut arr1 : [i64;1] = [0i64;1];
    match md5common::expect_int_array( &file_content[0..], idx_current, 1usize, & mut arr1 ) {
        Ok(v) => {
            idx_current = v;
            vert._weight_start = arr1[0] as u64;
        },
        Err(e) => assert!(false, e )
    }
    let mut arr2 : [i64;1] = [0i64;1];
    match md5common::expect_int_array( &file_content[0..], idx_current, 1usize, & mut arr2 ) {
        Ok(v) => {
            idx_current = v;
            vert._weight_count = arr2[0] as u64;
        },
        Err(e) => assert!(false, e )
    }
    Ok( ( idx_current, vert ) )
}

pub fn process_tri( file_content: &str, idx: usize, hm: & HashMap< &str, Token > ) -> Result< ( usize, Md5Tri ), & 'static str > {
    let mut idx_current = idx;
    let mut tri = Md5Tri {
        _index: 0u64,
        _vert_indices: [0u64;3],
    };
    let ( tok, kw_tok, _idx_s, _idx_e, idx_next ) = md5common::tokenize( &file_content[0..], idx_current, & hm );
    match tok {
        md5common::Token::Keyword => {
            match kw_tok {
                //expect: keyword: tri
                Some(Token::Tri) => {
                    idx_current = idx_next;
                },
                _ => return Err("unexpected token. Tri is not present.")
            }
        },
        _ => return Err("unexpected token. keyword is not present.")
    }
    let mut arr0 : [i64;1] = [0i64;1];
    match md5common::expect_int_array( &file_content[0..], idx_current, 1usize, & mut arr0 ) {
        Ok(v) => {
            idx_current = v;
            tri._index = arr0[0] as u64;
        },
        Err(e) => assert!(false, e )
    }
    let mut arr1 : [i64;3] = [0i64;3];
    match md5common::expect_int_array( &file_content[0..], idx_current, 3usize, & mut arr1 ) {
        Ok(v) => {
            idx_current = v;
            for i in 0..3 {
                tri._vert_indices[i] = arr1[i] as u64;
            }
        },
        Err(e) => assert!(false, e )
    }
    Ok( ( idx_current, tri ) )
}

pub fn process_weight( file_content: &str, idx: usize, hm: & HashMap< &str, Token > ) -> Result< ( usize, Md5Weight ), & 'static str > {
    let mut idx_current = idx;
    let mut weight = Md5Weight {
        _index: 0u64,
        _joint_index: 0u64,
        _weight_bias: 0f32,
        _pos: [0f32;3],
    };
    let ( tok, kw_tok, _idx_s, _idx_e, idx_next ) = md5common::tokenize( &file_content[0..], idx_current, & hm );
    match tok {
        md5common::Token::Keyword => {
            match kw_tok {
                //expect: keyword: weight
                Some(Token::Weight) => {
                    idx_current = idx_next;
                },
                _ => return Err("unexpected token. Weight is not present.")
            }
        },
        _ => return Err("unexpected token. keyword is not present.")
    }
    let mut arr0 : [i64;1] = [0i64;1];
    match md5common::expect_int_array( &file_content[0..], idx_current, 1usize, & mut arr0 ) {
        Ok(v) => {
            idx_current = v;
            weight._index = arr0[0] as u64;
        },
        Err(e) => assert!(false, e )
    }
    let mut arr1 : [i64;1] = [0i64;1];
    match md5common::expect_int_array( &file_content[0..], idx_current, 1usize, & mut arr1 ) {
        Ok(v) => {
            idx_current = v;
            weight._joint_index = arr1[0] as u64;
        },
        Err(e) => assert!(false, e )
    }
    let mut arr2 : [f32;1] = [0f32;1];
    match md5common::expect_float_array( &file_content[0..], idx_current, 1usize, & mut arr2 ) {
        Ok(v) => {
            idx_current = v;
            weight._weight_bias = arr2[0];
        },
        Err(e) => assert!(false, e )
    }
    match md5common::expect_parenl( &file_content[0..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    match md5common::expect_float_array( &file_content[0..], idx_current, 3usize, & mut weight._pos ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    match md5common::expect_parenr( &file_content[0..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    Ok( ( idx_current, weight ) )
}

pub fn parse( file_content: &str ) -> Result< Md5MeshRoot, & 'static str > {

    let mut hm_keywords = HashMap::new();
    hm_keywords.insert( "MD5Version", Token::Version );
    hm_keywords.insert( "commandline", Token::Commandline );
    hm_keywords.insert( "numJoints", Token::Numjoints );
    hm_keywords.insert( "numMeshes", Token::Nummeshes );
    hm_keywords.insert( "joints", Token::Joints );
    hm_keywords.insert( "mesh", Token::Mesh );
    hm_keywords.insert( "shader", Token::Shader );
    hm_keywords.insert( "numverts", Token::Numverts );
    hm_keywords.insert( "vert", Token::Vert );
    hm_keywords.insert( "numtris", Token::Numtris );
    hm_keywords.insert( "tri", Token::Tri );
    hm_keywords.insert( "numweights", Token::Numweights );
    hm_keywords.insert( "weight", Token::Weight );
    
    let mut idx = 0usize;
    let mut mesh_root = Md5MeshRoot::init();

    loop {
        let ( tok, kw_tok, idx_s, idx_e, mut idx_next ) = md5common::tokenize( &file_content[0..], idx, & mut hm_keywords );
        match tok {
            md5common::Token::End => {
                println!("reached end of content at index {}.", idx_next );
                break;
            },
            md5common::Token::Keyword => {
                match kw_tok {
                    Some(Token::Version) => {
                        let ( _tok2, _kw_tok2, idx_s2, idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm_keywords );
                        mesh_root._md5ver = u64::from_str( &file_content[idx_s2..idx_e2] ).expect("md5mesh parse version failed");
                        idx_next = idx_next2;
                    },
                    Some(Token::Commandline) => {
                        let ( _tok2, _kw_tok2, idx_s2, idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm_keywords );
                        mesh_root._cmdline = String::from_str( &file_content[idx_s2..idx_e2] ).expect("md5mesh parse cmdline failed");
                        idx_next = idx_next2;
                        println!("cmdline: {:?}.", mesh_root._cmdline );
                    },
                    Some(Token::Numjoints) => {
                        let ( _tok2, _kw_tok2, idx_s2, idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm_keywords );
                        mesh_root._numjoints = u64::from_str( &file_content[idx_s2..idx_e2] ).expect("md5mesh parse cmdline failed");
                        idx_next = idx_next2;
                        println!("numjoints: {:?}.", mesh_root._numjoints );
                    },
                    Some(Token::Nummeshes) => {
                        let ( _tok2, _kw_tok2, idx_s2, idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm_keywords );
                        mesh_root._nummeshes = u64::from_str( &file_content[idx_s2..idx_e2] ).expect("md5mesh parse cmdline failed");
                        idx_next = idx_next2;
                        println!("nummeshes: {:?}.", mesh_root._nummeshes );
                    },
                    Some(Token::Joints) => {
                        {
                            let ( tok2, _kw_tok2, _idx_s2, _idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm_keywords );
                            match tok2 {
                                md5common::Token::Bracel => (),
                                _ => panic!("invalid token. expected bracel.")
                            }
                            idx_next = idx_next2;
                        }
                        {
                            for _ in 0..mesh_root._numjoints {
                                match process_joint( &file_content[0..], idx_next, & mut hm_keywords ) {
                                    Ok( ( v, joint ) ) => {
                                        idx_next = v;
                                        mesh_root._joints.push( joint );
                                    },
                                    Err(e) => assert!(false, e )
                                }
                            }
                        }
                        {
                            let ( tok2, _kw_tok2, _idx_s2, _idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm_keywords );
                            match tok2 {
                                md5common::Token::Bracer => (),
                                _ => panic!("invalid token. expected bracel.")
                            }
                            idx_next = idx_next2;
                        }
                    },
                    Some(Token::Mesh) => {
                        match process_mesh( &file_content[0..], idx_next, & hm_keywords ) {
                            Ok( ( index, m ) ) => {
                                idx_next = index;
                                mesh_root._meshes.push( m );
                            },
                            Err(e) => assert!(false, e)
                        }
                    },
                    _ => panic!("unexpected token detected at index {:?}. input: {:?}", idx_next, &file_content[idx_s..idx_e] ),
                }
            },
            md5common::Token::Invalid => panic!("invalid token detected at index {:?}. input: {:?}", idx_next, &file_content[idx_s..idx_e] ),
            _ => ()
        }
        idx = idx_next;
    }
    assert!( mesh_root._numjoints ==  mesh_root._joints.len() as u64 );
    assert!( mesh_root._nummeshes ==  mesh_root._meshes.len() as u64 );
    Ok( mesh_root )
}
