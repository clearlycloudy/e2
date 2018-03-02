#![allow(non_snake_case)]
#![allow(unused_variables)]

extern crate pretty_env_logger;
extern crate nom;

use std::str;
use std::str::FromStr;

use self::nom::digit;
use implement::file::md5mesh::*;

named!( shader_path< &str, String >,
        do_parse!(
            ws!( tag!("shader") ) >>
            path: delimited!(
                tag!("\""),
                take_until!("\""),
                tag!("\"")
            ) >>
            ( path.to_string() )
        )
);

//weights
named!( md5mesh_weight< &str, Md5Weight >,
        do_parse!(
            ws!( tag!("weight") ) >> 
            idx: map_res!( ws!(digit), FromStr::from_str ) >>
            joint_idx: map_res!( ws!(digit), FromStr::from_str ) >>
            weight_bias: ws!(nom::float_s) >>
            ws!(tag!("(")) >>
            pos0: ws!(nom::float_s) >>
            pos1: ws!(nom::float_s) >>
            pos2: ws!(nom::float_s) >>
            ws!(tag!(")")) >>    
            (
                Md5Weight {
                    _index: idx,
                    _joint_index: joint_idx,
                    _weight_bias:  weight_bias,
                    _pos: [ pos0, pos1, pos2 ],
                }
            )
        )
);

named!( md5mesh_weights< &str, Vec<Md5Weight> >,
        do_parse!(
            ws!( tag!("numweights") ) >>
            count_weights: map_res!( ws!(digit), FromStr::from_str ) >>
            weights: count!(
                md5mesh_weight
                ,count_weights ) >>
            ( weights )
        )
);

named!( md5mesh_vert< &str, Md5Vert >,
        do_parse!(
            ws!( tag!("vert") ) >> 
            idx: map_res!( ws!(digit), FromStr::from_str ) >>
            ws!(tag!("(")) >>
            v0: ws!(nom::float_s) >>
            v1: ws!(nom::float_s) >>
            ws!(tag!(")")) >>    
            v2: map_res!( ws!(digit), FromStr::from_str ) >>
            v3: map_res!( ws!(digit), FromStr::from_str ) >>
            (
                Md5Vert {
                    _index: idx,
                    _tex_coords: [ v0, v1 ],
                    _weight_start: v2,
                    _weight_count: v3,
                    _normal: [0f32;3],
                    _pos: [0f32;3],
                }
            )
        )
);

named!( md5mesh_verts< &str, Vec<Md5Vert> >,
        do_parse!(
            ws!( tag!("numverts") ) >>
            count_verts: map_res!( ws!(digit), FromStr::from_str ) >>
            verts: count!(
                md5mesh_vert
                ,count_verts ) >>
            ( verts )
        )
);

named!( md5mesh_tri< &str, Md5Tri >,
        do_parse!(
            ws!( tag!("tri") ) >> 
            idx: map_res!( ws!(digit), FromStr::from_str ) >>
            v0: map_res!( ws!(digit), FromStr::from_str ) >>
            v1: map_res!( ws!(digit), FromStr::from_str ) >>
            v2: map_res!( ws!(digit), FromStr::from_str ) >>
            (
                Md5Tri {
                    _index: idx,
                    _vert_indices: [ v0, v1, v2 ]
                }
            )
        )
);
        
named!( md5mesh_tris< &str, Vec< Md5Tri > >,
        do_parse!(
            ws!( tag!("numtris") ) >>
            count_tris: map_res!( ws!(digit), FromStr::from_str ) >>
            tris: count!(
                md5mesh_tri
                ,count_tris ) >>
            ( tris )
        )
);

named!( md5mesh_parse_opening< &str, () >,
        do_parse!(
            ws!( tag!("mesh") ) >>
            ws!( tag!("{") ) >>
            ()
        )
);
named!( md5mesh_parse_closing< &str, () >,
        do_parse!(
            ws!( tag!("}") ) >>
            ()
        )
);

named!( peek_shader< &str, &str >,
        peek!(
            ws!( tag!("shader") )
        )
);

named!( peek_verts< &str, &str >,
        peek!(
            ws!( tag!("numverts") )
        )
);

named!( peek_tris< &str, &str >,
        peek!(
            ws!( tag!("numtris") )
        )
);

named!( peek_weights< &str, &str >,
        peek!(
            ws!( tag!("numweights") )
        )
);

named!( peek_version< &str, &str >,
        peek!(
            ws!( tag!("MD5Version") )
        )
);

named!( md5_version< &str, isize >,
        do_parse!(
            ws!( tag!("MD5Version") ) >>
            version: map_res!( ws!(digit), FromStr::from_str ) >>
            ( version )
        )
);

named!( peek_commandline< &str, &str >,
        peek!(
            ws!( tag!("commandline") )
        )
);

named!( md5_commandline< &str, String >,
        do_parse!(
            ws!( tag!("commandline") ) >>
            cmd: delimited!(
                tag!("\""),
                take_until!("\""),
                tag!("\"")
            ) >>
            ( cmd.to_string() )
        )
);

named!( peek_numJoints< &str, &str >,
        peek!(
            ws!( tag!("numJoints") )
        )
);

named!( md5_numJoints< &str, isize >,
        do_parse!(
            ws!( tag!("numJoints") ) >>
            num: map_res!( ws!(digit), FromStr::from_str ) >>
            ( num )
        )
);

named!( peek_numMeshes< &str, &str >,
        peek!(
            ws!( tag!("numMeshes") )
        )
);

named!( md5_numMeshes< &str, isize >,
        do_parse!(
            ws!( tag!("numMeshes") ) >>
            num: map_res!( ws!(digit), FromStr::from_str ) >>    
            ( num )
        )
);

named!( peek_joints< &str, &str >,
        peek!(
            ws!( tag!("joints") )
        )
);


named!( signed_num< &str, &str >,
        recognize!(
            do_parse!(
                sgn: alt!( tag!("+") | tag!("-") | tag!("") ) >>
                d: digit >> ()
            )
        )
);

named!( md5mesh_joint< &str, Md5Joint >,
        do_parse!(
            name: ws!(delimited!(
                tag!("\""),
                take_until!("\""),
                tag!("\"")
            ) ) >>
            idx: map_res!(ws!(signed_num), <isize as FromStr>::from_str ) >>   
            ws!(tag!("(")) >>
            p0: ws!(nom::float_s) >>
            p1: ws!(nom::float_s) >>
            p2: ws!(nom::float_s) >>
            ws!(tag!(")")) >>
            ws!(tag!("(")) >>
            o0: ws!(nom::float_s) >>
            o1: ws!(nom::float_s) >>
            o2: ws!(nom::float_s) >>
            ws!(tag!(")")) >>
            (
                Md5Joint {
                    _name: name.to_string(),
                    _parent_index: idx as i64,
                    _pos: [ p0, p1, p2 ],
                    _orient: [ o0, o1, o2 ],
                    _rot: Default::default(),
                }
            )
        )
);

named!( md5mesh_joints_opening< &str, &str >,
        do_parse!(
            ws!( tag!("joints") ) >>
            a: ws!( tag!("{") ) >>
            ( a )    
        )
);

named!( md5mesh_joints_closing< &str, () >,
        do_parse!(
            ws!( tag!("}") ) >>
            ()    
        )
);

named!( peek_comments< &str, &str >,
        peek!(
            ws!( tag!("//") )
        )
);

named!( consume_comments_start< &str, () >,
        do_parse!(
            take_until_either!( "\n\r" ) >>
            tag!("//") >>
            ()
        )
);

named!( consume_comments< &str, &str >,
        take_until_either!( "\n\r" )
);

named!( consume_newline< &str, &str >,
        not!( take_until_and_consume!( "\n\r" ) )
);

fn parse_mesh( input: & str ) -> Result< ( & str, Md5Mesh ), & 'static str > {
    
    let mut buf = input;
    
    match md5mesh_parse_opening( buf ) {
        nom::IResult::Done( i, _ ) => {
            buf = i;
        },
        other => {
            return Err( "no mesh opening token found" )
        }
    };

    let mut shader : Option< String > = None;
    let mut verts : Option< Vec< Md5Vert > > = None;
    let mut tris : Option< Vec< Md5Tri > > = None;
    let mut weights : Option< Vec< Md5Weight > > = None;

    loop {
        
        let mut progress = false;

        match peek_and_consume_comments( buf ) {
            Some(x) => {
                buf = x;
                progress = true;
            },
            _ => {},
        }

        match peek_shader( buf ) {
            nom::IResult::Done( _, _ ) => {
                match shader_path( buf ) {
                    nom::IResult::Done( i, o ) => {
                        shader = Some( o );
                        buf = i;
                        progress = true;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        match peek_verts( buf ) {
            nom::IResult::Done( _, _ ) => {
                match md5mesh_verts( buf ) {
                    nom::IResult::Done( i, o ) => {
                        verts = Some( o );
                        buf = i;
                        progress = true;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        match peek_tris( buf ) {
            nom::IResult::Done( _, _ ) => {
                match md5mesh_tris( buf ) {
                    nom::IResult::Done( i, o ) => {
                        tris = Some( o );
                        buf = i;
                        progress = true;
                    },
                    _ => {},
                }
            },
            _ => {},
        }
        
        match peek_weights( buf ) {
            nom::IResult::Done( _, _ ) => {
                match md5mesh_weights( buf ) {
                    nom::IResult::Done( i, o ) => {
                        weights = Some( o );
                        buf = i;
                        progress = true;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        if !progress {
            break;
        }
    }

    match md5mesh_parse_closing( buf ) {
        nom::IResult::Done( i, _ ) => {
            buf = i;
        },
        _ => { return Err("mesh closing token not found") },
    }

    match ( shader, verts, tris, weights ) {
        ( Some(s), Some(v), Some(t), Some(w) ) => {
            Ok( ( buf, Md5Mesh {
                _shader: s,
                _numverts: v.len() as u64,
                _numtris: t.len() as u64,
                _numweights: w.len() as u64,
                _verts: v,
                _tris: t,
                _weights: w,
            } ) )
        },
        _ => {
            Err( "Mesh parse unsuccessful" )
        }
    }
}

fn peek_and_consume_comments( mut input: & str ) -> Option< & str > {
    match peek_comments( input ) {
        nom::IResult::Done( i, o ) => {
            match consume_comments_start( i ) {
                nom::IResult::Done( j, o ) => {
                    input = j;
                },
                _ => {},
            }
            match consume_comments( input ) {
                nom::IResult::Done( j, o ) => {
                    input = j;
                },
                _ => {},
            }
            match consume_newline( input ) {
                nom::IResult::Done( k, o ) => {
                    input = k;
                },
                _ => {},
            }
            return Some( input )
        },
        _ => {},
    }
    None
}

pub fn parse( file_content: &str ) -> Result< Md5MeshRoot, & 'static str > {

    // let input = file_open( "core/asset/md5/qshambler.md5mesh" ).expect( "input file invalid" );

    let mut buf = file_content;

    let mut version = None;
    let mut cmdline = None;
    let mut num_joints = None;
    let mut num_meshes = None;

    let mut joints : Vec< Md5Joint > = vec![];
    let mut meshes : Vec< Md5Mesh > = vec![];
    
    loop {

        let mut progress = false;
        
        match peek_version( buf ) {
            nom::IResult::Done( _, _ ) => {
                match md5_version( buf ) {
                    nom::IResult::Done( i, o ) => {
                        version = Some( o );
                        buf = i;
                        progress = true;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        match peek_commandline( buf ) {
            nom::IResult::Done( _, _ ) => {
                match md5_commandline( buf ) {
                    nom::IResult::Done( i, o ) => {
                        cmdline = Some( o );
                        buf = i;
                        progress = true;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        match peek_numJoints( buf ) {
            nom::IResult::Done( _, _ ) => {
                match md5_numJoints( buf ) {
                    nom::IResult::Done( i, o ) => {
                        debug!( "num joints: {:?}", o );
                        num_joints = Some( o );
                        buf = i;
                        progress = true;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        match peek_numMeshes( buf ) {
            nom::IResult::Done( _, _ ) => {
                match md5_numMeshes( buf ) {
                    nom::IResult::Done( i, o ) => {
                        debug!( "num meshes: {:?}", o );
                        num_meshes = Some( o );
                        buf = i;
                        progress = true;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        match peek_joints( buf ) {
            nom::IResult::Done( _, _ ) => {
                match md5mesh_joints_opening( buf ) {
                    nom::IResult::Done( i, o ) => {
                        progress = true;
                        buf = i;
                    },
                    _ => {
                        return Err( "joint opening token not found" )
                    },
                }
                match num_joints {
                    None => {
                        return Err( "num joints not specified at point of point parsing" )
                    },
                    _ => {},
                }
                
                let n = num_joints.unwrap();
                let mut count = 0;
                while count < n {
                    match peek_and_consume_comments( buf ) {
                        Some(x) => {
                            progress = true;
                            buf = x;
                            continue;
                        },
                        _ => {}
                    }
                    match md5mesh_joint( buf ) {
                        nom::IResult::Done( i, o ) => {
                            buf = i;
                            progress = true;
                            joints.push( o );
                        },
                        _ => {
                            return Err("joint parse unsuccessful")
                        },
                    }

                    count += 1;
                }                

                match peek_and_consume_comments( buf ) {
                    Some(x) => {
                        buf = x;
                    },
                    _ => {}
                }

                match md5mesh_joints_closing( buf ) {
                    nom::IResult::Done( i, o ) => {
                        progress = true;
                        buf = i;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        if !progress {
            break;
        }
    }

    match peek_and_consume_comments( buf ) {
        Some(x) => { buf = x; },
        _ => {},
    }

    match num_meshes {
        None => {
            return Err( "num meshes not present" );
        },
        _ => {},
    }

    for _ in 0..num_meshes.unwrap() {
        match parse_mesh( buf ) {
            Ok( ( b, m ) ) => {
                buf = b;
                meshes.push( m );                
            },
            Err(e) => {
                return Err(e)
            }
        };
    }

    debug!("num joints: {:?}", joints.len() );
    debug!("num meshes: {:?}", meshes.len() );

    if let None = version {
        return Err( "version not present" );
    }
    if let None = cmdline {
        return Err( "cmdline not present" );
    }
    match num_joints {
        Some(x) => {
            if x != joints.len() as isize {
                assert_eq!( x, joints.len() as isize );
            }
        }
        _ => {
            return Err( "num joints not present" );
        },
    }

    assert_eq!( num_meshes.unwrap(), meshes.len() as isize );

    Ok( Md5MeshRoot {
        _md5ver: version.unwrap() as u64,
        _cmdline: cmdline.unwrap(),
        _numjoints: num_joints.unwrap() as u64,
        _nummeshes: num_meshes.unwrap() as u64,
        _joints: joints,
        _meshes: meshes,
    } )
}
