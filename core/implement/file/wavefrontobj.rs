#![allow(non_snake_case)]
#![allow(unused_variables)]

extern crate pretty_env_logger;
extern crate nom;

use std::str;
use std::str::FromStr;

use self::nom::digit;

use interface::i_wavefront::obj::*;
use interface::i_file::IParseStr;

// named!(end_of_line, alt!(
//     nom::eof!()
//         |
//     nom::eol
//         |
//     comment  // handle end of line comments - these are not kept
// ));

// named!(comment, delimited!(
//     tag!("#"),
//     take_until!("\n"),
//     alt!( eof!() | nom::eol )
// ));

named!( single_word< &str, &str >,
        take_until_either!(" \n\r\t")
);

//todo: add "_" to the set of allowable characters
named!( any_nonwhitespace< &str, &str >,
        do_parse!(
            word: ws!(nom::alphanumeric) >>
            (
                word
            )
        )
);

named!( parse_mtllib< &str, String >,
        do_parse!(
            ws!( tag!("mtllib") ) >>
            path: single_word >>
            ( path.to_string() )
        )
);

named!( parse_material< &str, String >,
        do_parse!(
            ws!( tag!("usemtl") ) >>
            mtl: single_word >>
            ( mtl.to_string() )
        )
);

named!( parse_g< &str, String >,
        do_parse!(
            ws!( tag!("g") ) >>
            path: single_word >>
            ( path.to_string() )
        )
);

named!( parse_s< &str, String >,
        do_parse!(
            ws!( tag!("s") ) >>
            path: single_word >>
            ( path.to_string() )
        )
);

named!( peek_s< &str, &str >,
        peek!(
            ws!( tag!("s") )
        )
);

named!( v< &str, [ f32; 3] >,
        do_parse!(
            ws!(tag!("v")) >>
            pos0: alt!( ws!(nom::float_s) | map_res!(ws!(digit),FromStr::from_str)) >>
            pos1: alt!( ws!(nom::float_s) | map_res!(ws!(digit),FromStr::from_str)) >>
            pos2: alt!( ws!(nom::float_s) | map_res!(ws!(digit),FromStr::from_str)) >>
            ( [ pos0, pos1, pos2 ] )
        )
);

named!( vn< &str, [ f32; 3] >,
        do_parse!(
            ws!( tag!("vn") ) >>
            n0: alt!( ws!(nom::float_s) | map_res!(ws!(digit),FromStr::from_str)) >>
            n1: alt!( ws!(nom::float_s) | map_res!(ws!(digit),FromStr::from_str)) >>
            n2: alt!( ws!(nom::float_s) | map_res!(ws!(digit),FromStr::from_str)) >>
            ( [ n0, n1, n2 ] )
        )
);

named!( vt< &str, [ f32; 2] >,
        do_parse!(
            ws!( tag!("vt") ) >>
            vt0: alt!( ws!(nom::float_s) | map_res!(ws!(digit),FromStr::from_str)) >>
            vt1: alt!( ws!(nom::float_s) | map_res!(ws!(digit),FromStr::from_str)) >>
            ( [ vt0, vt1 ] )
        )
);


named!( f_vtn< &str, Face >,
        do_parse!(
            ws!( tag!("f") ) >>
            v0: map_res!(ws!(digit), FromStr::from_str) >>
            tag!("/") >>
            tc0: map_res!(ws!(digit), FromStr::from_str) >>
            tag!("/") >>
            n0: map_res!(ws!(digit), FromStr::from_str) >>

            v1: map_res!(ws!(digit), FromStr::from_str) >>
            tag!("/") >>
            tc1: map_res!(ws!(digit), FromStr::from_str) >>
            tag!("/") >>
            n1: map_res!(ws!(digit), FromStr::from_str) >>

            v2: map_res!(ws!(digit), FromStr::from_str) >>
            tag!("/") >>
            tc2: map_res!(ws!(digit), FromStr::from_str) >>
            tag!("/") >>
            n2: map_res!(ws!(digit), FromStr::from_str) >>
                
            (
                Face {
                    _vert_index: [ v0, v1, v2 ],
                    _tc_index: Some( [ tc0, tc1, tc2 ] ),
                    _normal_index: Some( [ n0, n1, n2 ] ),
                }
            )
        )
);

named!( f_vt< &str, Face >,
        do_parse!(
            ws!( tag!("f") ) >>
            v0: map_res!(ws!(digit), FromStr::from_str) >>
            tag!("/") >>
            tc0: map_res!(ws!(digit), FromStr::from_str) >>

            v1: map_res!(ws!(digit), FromStr::from_str) >>
            tag!("/") >>
            tc1: map_res!(ws!(digit), FromStr::from_str) >>

            v2: map_res!(ws!(digit), FromStr::from_str) >>
            tag!("/") >>
            tc2: map_res!(ws!(digit), FromStr::from_str) >>
                
            (
                Face {
                    _vert_index: [ v0, v1, v2 ],
                    _tc_index: Some( [ tc0, tc1, tc2 ] ),
                    _normal_index: None,
                }
            )
        )
);

named!( f_vn< &str, Face >,
        do_parse!(
            ws!( tag!("f") ) >>
            v0: map_res!(ws!(digit), FromStr::from_str) >>
            ws!(tag!("/")) >>
            ws!(tag!("/")) >>
            n0: map_res!(ws!(digit), FromStr::from_str) >>

            v1: map_res!(ws!(digit), FromStr::from_str) >>
            ws!(tag!("/")) >>
            ws!(tag!("/")) >>
            n1: map_res!(ws!(digit), FromStr::from_str) >>

            v2: map_res!(ws!(digit), FromStr::from_str) >>
            ws!(tag!("/")) >>
            ws!(tag!("/")) >>
            n2: map_res!(ws!(digit), FromStr::from_str) >>
                
            (
                Face {
                    _vert_index: [ v0, v1, v2 ],
                    _tc_index: None,
                    _normal_index: Some( [ n0, n1, n2 ] ),
                }
            )
        )
);

named!( f_v< &str, Face >,
        do_parse!(
            ws!( tag!("f") ) >>
            v0: map_res!(ws!(digit), FromStr::from_str) >>

            v1: map_res!(ws!(digit), FromStr::from_str) >>

            v2: map_res!(ws!(digit), FromStr::from_str) >>
                
            (
                Face {
                    _vert_index: [ v0, v1, v2 ],
                    _tc_index: None,
                    _normal_index: None,
                }
            )
        )
);

named!( f< &str, Face >,
        do_parse!(
            f: alt!( f_vtn | f_vt | f_vn |f_v ) >>
            ( f )
        )
);

named!( peek_comments< &str, &str >,
        peek!(
            ws!( tag!("#") )
        )
);

named!( consume_comments_start< &str, () >,
        do_parse!(
            ws!(tag!("#")) >>
            take_until_either!( "\n\r" ) >>
            ()
        )
);

named!( consume_comments< &str, &str >,
        take_until_either!( "\n\r" )
);

named!( consume_newline< &str, &str >,
        not!( take_until_and_consume!( "\n\r" ) )
);

named!( peek_g< &str, &str >,
        peek!(
            ws!( tag!("g") )
        )
);

named!( peek_vertex< &str, &str >,
        peek!(
            ws!( tag!("v ") )
        )
);

named!( peek_texture_coord< &str, &str >,
        peek!(
            ws!( tag!("vt") )
        )
);

named!( peek_vertex_normal< &str, &str >,
        peek!(
            ws!( tag!("vn") )
        )
);

named!( peek_face< &str, &str >,
        peek!(
            ws!( tag!("f") )
        )
);

named!( peek_material< &str, &str >,
        peek!(
            ws!( tag!("usemtl") )
        )
);

named!( peek_mtllib< &str, &str >,
        peek!(
            ws!( tag!("mtllib") )
        )
);

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

fn parse_group( mut buf: & str ) -> Result< ( & str, Option<Group> ), & 'static str > {

    let mut group = None;
    let mut material = None;
    let mut vertices = vec![];
    let mut tx_coords = vec![];
    let mut normals = vec![];
    let mut faces = vec![];
    
    loop {
        // println!("Loop inner: {:?}", &buf[..20] );
        let mut progress = false;

        match peek_and_consume_comments( buf ) {
            Some(x) => {
                buf = x;
                continue;
            },
            _ => {}
        }

        match peek_s( buf ) {
            nom::IResult::Done( i, o ) => {
                match parse_s( buf ) {
                    nom::IResult::Done( i, o ) => {
                        buf = i;
                        progress = true;
                    },
                    _ => {},
                }
            },
            _ => {},
        }
        
        match peek_g( buf ) {
            nom::IResult::Done( i, o ) => {
                if let Some(_) = group {
                    break;
                }
                match parse_g( buf ) {
                    nom::IResult::Done( i, o ) => {
                        buf = i;
                        group = Some(o);
                        progress = true;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        match peek_vertex( buf ) {
            nom::IResult::Done( i, o ) => {
                match v( buf ) {
                    nom::IResult::Done( i, o ) => {
                        buf = i;
                        vertices.push( o );
                        progress = true;
                    },
                    _ => {
                        return Err("parse vertex coord")
                    },
                }
            },
            _ => {},
        }

        match peek_texture_coord( buf ) {
            nom::IResult::Done( i, o ) => {
                match vt( buf ) {
                    nom::IResult::Done( i, o ) => {
                        buf = i;
                        tx_coords.push( o );
                        progress = true;
                    },
                    _ => {
                        return Err("parse texture coord unsuccessful")
                    },
                }
            },
            _ => {},
        }

        match peek_vertex_normal( buf ) {
            nom::IResult::Done( i, o ) => {
                match vn( buf ) {
                    nom::IResult::Done( i, o ) => {
                        buf = i;
                        normals.push( o );
                        progress = true;
                    },
                    _ => {
                        return Err("parse vertex normal unsuccessful")
                    },
                }
            },
            _ => {},
        }

        match peek_face( buf ) {
            nom::IResult::Done( i, o ) => {
                match f( buf ) {
                    nom::IResult::Done( i, o ) => {
                        buf = i;
                        faces.push( o );
                        progress = true;
                    },
                    _ => {
                        return Err("parse face unsuccessful")
                    },
                }
            },
            _ => {},
        }

        match peek_material( buf ) {
            nom::IResult::Done( i, o ) => {
                match parse_material( buf ) {
                    nom::IResult::Done( i, o ) => {
                        buf = i;
                        material = Some( o );
                        progress = true;
                    },
                    _ => {
                        // return Err("parse material unsuccessful")
                    },
                }
            },
            _ => {},
        }
            
        if !progress {
            break;
        }
    }

    // if let None = group {
    //     return Err( "group name missing" )
    // }
    // if let None = material {
    //     return Err( "material name missing" )
    // }

    if vertices.len() == 0 {
        return Ok( ( buf, None ) )
    }

    Ok(
        ( buf,
          Some( Group {
              _name: None,
              _group: group,
              _material: material,
              _verts: vertices,
              _vert_normals: normals,
              _faces: faces,
              _texture_coords: tx_coords,
          } )
        )
    )
}

pub fn parse( input: & str ) -> Result< Collection, & 'static str > {
    
    let mut buf = input;

    let mut groups = vec![];

    let mut mtllib = None;

    loop {
        match peek_and_consume_comments( buf ) {
            Some(x) => {
                buf = x;
                continue;
            },
            _ => {}
        }
        break;
    }

    match peek_mtllib( buf ) {
        nom::IResult::Done( i, o ) => {
            match parse_mtllib( buf ) {
                nom::IResult::Done( i, o ) => {
                    buf = i;
                    mtllib = Some( o );
                },
                _ => {
                    return Err("parse mtllib unsuccessful")
                },
            }
        },
        _ => {},
    }

    loop {
        
        // println!("Loop");
        let mut progress = false;
        match parse_group( buf ) {
            Ok( ( i, g ) ) => {
                match g {
                    Some(o) => {
                        // println!( "{:?}", o );
                        groups.push( o );
                        buf = i;
                        progress = true;
                    },
                    _ => {
                        break;
                    }
                }
            },
            Err( e ) => {
                return Err( e )
            },
        }
        if !progress {
            break;
        }
    }

    if let None = mtllib {
        return Err( "mtllib missing" )
    }

    Ok(
        Collection {
            _mtllib: mtllib.unwrap(),
            _groups: groups,
        }
    )
}
